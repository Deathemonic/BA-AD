import json
import sys
import asyncio
from pathlib import Path
from typing import List, Dict, Tuple

from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.table import Table
from rich.text import Text
from rich.live import Live

from rapidfuzz import process, fuzz

from .ResourceDownloader import ResourceDownloader


class CatalogList:
    def __init__(self, root_path: Path, output_path: Path):
        self.root = root_path
        self.console = Console()
        self.score_cutoff = 85
        self.query = ""
        self.selected_index = 0
        self.scroll_offset = 0
        self.visible_items = self.console.height - 8
        self.downloader = ResourceDownloader(output=output_path)
        
        game_files_path = self.downloader.catalog_parser.cache_dir / 'GameFiles.json'
        if not game_files_path.exists():
            self.console.print("[yellow]Initializing game files...[/yellow]")
            self.downloader.initialize_download()
            
        self.all_items = self._load_all_items()
        
    def _format_size(self, size: int) -> str:
        if size == 0:
            return ""
        for unit in ['B', 'KB', 'MB', 'GB']:
            if size < 1024:
                return f"{size:.1f}{unit}"
            size /= 1024
        return f"{size:.1f}TB"
        
    def _load_catalogs(self) -> Dict[str, List[dict]]:
        cache_dir = self.downloader.catalog_parser.cache_dir
        paths = {
            'AssetBundles': (cache_dir / 'bundleDownloadInfo.json', 'BundleFiles'),
            'MediaResources': (cache_dir / 'MediaCatalog.json', 'MediaResources'),
            'TableBundles': (cache_dir / 'TableCatalog.json', 'TableBundles')
        }
        
        return {
            category: self._load_catalog_items(category, path, key)
            for category, (path, key) in paths.items()
            if path.exists()
        }

    def _load_catalog_items(self, category: str, path: Path, key: str) -> List[dict]:
        with open(path) as f:
            data = json.load(f)
            
        if category == 'AssetBundles':
            return [{'name': item['Name'], 'size': item.get('Size', 0)} for item in data.get(key, [])]
            
        if category == 'MediaResources':
            return [{'name': Path(item['path']).name, 'size': item.get('bytes', 0)} for item in data.get(key, {}).values()]
            
        return [{'name': name, 'size': data.get('size', 0)} for name, data in data.get(key, {}).items()]

    def _create_table(self, items: List[Tuple[str, str, int]]) -> Table:
        table = Table(show_header=True, header_style="bold magenta", expand=True, box=None)
        table.add_column("Category", style="cyan", width=15)
        table.add_column("Name", style="green")
        table.add_column("Size", justify="right", style="yellow", width=10)

        total_items = len(items)
        self.visible_items = min(self.console.height - 8, total_items)
        
        self.selected_index = min(self.selected_index, total_items - 1)
        self.scroll_offset = min(self.scroll_offset, total_items - self.visible_items)
        self.scroll_offset = max(0, self.scroll_offset)

        start_idx = self.scroll_offset
        end_idx = min(start_idx + self.visible_items, total_items)
        
        for idx in range(start_idx, end_idx):
            category, name, size = items[idx]
            style = "reverse" if idx == self.selected_index else ""
            size_text = self._format_size(size)
            
            table.add_row(
                Text(category, style=style),
                Text(name, style=style),
                Text(size_text, style=style)
            )

        return table

    def _filter_items(self) -> List[Tuple[str, str, int]]:
        if not self.query:
            return self.all_items
            
        query = self.query.lower()
        filtered_items = []
        
        for category, name, size in self.all_items:
            if query in name.lower():
                filtered_items.append((category, name, size))
                
        if not filtered_items:
            choices = [(cat, name) for cat, name, _ in self.all_items]
            matches = process.extract(
                query=query,
                choices=choices,
                scorer=fuzz.token_sort_ratio,
                score_cutoff=self.score_cutoff,
                limit=None
            )
            filtered_items = [(cat, name, size) for (cat, name), _, _ in matches for _, n, s in [next((i for i in self.all_items if i[0] == cat and i[1] == name), (None, None, 0))]]
            
        return sorted(filtered_items, key=lambda x: x[2], reverse=True)

    def _load_all_items(self) -> List[Tuple[str, str, int]]:
        catalogs = self._load_catalogs()
        return [
            (category, item['name'], item['size'])
            for category, items in catalogs.items()
            for item in items
        ]
    
    def _get_char(self) -> str:
        if sys.platform == "win32":
            import msvcrt
            if not msvcrt.kbhit():
                return ""
            
            char = msvcrt.getch()

            if char in [b'\x03', b'\x1b']:  # Ctrl+C, ESC
                return '\x1b'
            
            elif char == b'\x08':  # Backspace
                return '\x08'
            
            elif char == b'\xe0':
                char = msvcrt.getch()

                if char == b'H':  # Up arrow
                    return '\x1b[A'
                
                elif char == b'P':  # Down arrow
                    return '\x1b[B'
                
            return char.decode('utf-8', errors='ignore')
        
        if sys.platform == "posix":
            import tty
            import termios
            import select
            
            if not select.select([sys.stdin], [], [], 0)[0]:
                return ""
                
            fd = sys.stdin.fileno()
            old_settings = termios.tcgetattr(fd)

            try:
                tty.setraw(sys.stdin.fileno())
                char = sys.stdin.read(1)
                
                if char == '\x1b': # Ctrl+C, ESC
        
                    if select.select([sys.stdin], [], [], 0.1)[0]:
                        char += sys.stdin.read(2)

            finally:
                termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
                
            return char

    def _handle_input(self, char: str, live: Live) -> bool:
        if not char:
            return True
            
        if char == '\x1b':  # ESC
            return False
            
        if char == '\x08' or char == '\x7f':  # Backspace
            self.query = self.query[:-1]
            self.selected_index = 0
            self.scroll_offset = 0
            
        elif char == '\x1b[A':  # Up arrow
            self.selected_index = max(0, self.selected_index - 1)

            if self.selected_index < self.scroll_offset:
                self.scroll_offset = self.selected_index
                
        elif char == '\x1b[B':  # Down arrow
            filtered_items = self._filter_items()
            max_index = len(filtered_items) - 1
            self.selected_index = min(self.selected_index + 1, max_index)

            if self.selected_index >= self.scroll_offset + self.visible_items:
                self.scroll_offset += 1

        elif char == '\r':
            filtered_items = self._filter_items()
            self._download_selected_item(filtered_items, live)

        else:
            self.query += char
            self.selected_index = 0
            self.scroll_offset = 0
            
        return True
    
    def _download_selected_item(self, filtered_items: List[Tuple[str, str, int]], live: Live) -> None:
        if not filtered_items or self.selected_index >= len(filtered_items):
            return

        category, name, _ = filtered_items[self.selected_index]

        live.stop()

        game_files = self.downloader.initialize_download()
        
        selected_file = None
        category_key = category if category in ['AssetBundles', 'MediaResources', 'TableBundles'] else None
            
        if not (category_key and category_key in game_files):
            return

        for file in game_files[category_key]:
            file_name = (
                file.get('Name') or 
                Path(file.get('path', '')).name or 
                Path(file['url']).name
            )

            if file_name == name:
                selected_file = file
                break

        if selected_file:
            single_file = {category_key: [selected_file]}    
            asyncio.run(self.downloader._download_all_categories(
                single_file, 
                [category_key]
            ))

            self._get_char()
        
        live.start()

    def show(self):
        layout = Layout()
        layout.split_column(
            Layout(name="search", size=3),
            Layout(name="content")
        )
        
        with Live(layout, console=self.console, screen=True, refresh_per_second=60) as live:
            for _ in iter(bool, True):
                filtered_items = self._filter_items()
                
                layout["search"].update(Panel(
                    Text(f"> {self.query}", style="bold blue"),
                    title=f"[{len(filtered_items)} matches] [Press ESC to exit]",
                    style="blue"
                ))
                
                table = self._create_table(filtered_items)
                layout["content"].update(Panel(table))
                
                if not self._handle_input(self._get_char(), live):
                    break
