import asyncio
from pathlib import Path

from rich.console import Console
from rich.text import Text
from rich.live import Live
from rapidfuzz import process, fuzz

from ..helpers.json import load_json
from ..helpers.interface import (
    format_size,
    create_table,
    create_layout,
    create_panel,
    get_character_input
)
from .downloader import ResourceDownloader


class List:
    def __init__(self, root_path: Path, output: Path = None, update: bool = False):
        self.root = root_path
        self.console = Console()
        self.score_cutoff = 85
        self.query = ""
        self.selected_index = 0
        self.scroll_offset = 0
        self.visible_items = self.console.height - 8
        
        self.downloader = ResourceDownloader(output=output, update=update)

        self.console.print("[cyan]Fetching catalogs...[/cyan]")
        self.downloader.catalog_parser.fetch_catalogs()

        game_files_path = self.downloader.catalog_parser.cache_dir / 'GameFiles.json'
        if not game_files_path.exists():
            self.console.print("[yellow]Initializing game files...[/yellow]")
            self.downloader.initialize_download()

        self.all_items = self._load_all_items()
        
    def _load_catalogs(self) -> dict:
        cache_dir = self.downloader.catalog_parser.cache_dir
        
        catalog_configs = {
            'AssetBundles': (cache_dir / 'bundleDownloadInfo.json', 'BundleFiles'),
            'MediaResources': (cache_dir / 'MediaCatalog.json', 'MediaResources'),
            'TableBundles': (cache_dir / 'TableCatalog.json', 'TableBundles')
        }
        
        result = {}
        for category, (path, key) in catalog_configs.items():
            if not path.exists():
                continue
                
            result[category] = self._load_catalog_items(category, path, key)
            
        return result

    def _load_catalog_items(self, category: str, path: Path, key: str) -> list:
        data = load_json(path)
            
        if category == 'AssetBundles':
            return self._load_asset_bundles(data, key)
            
        if category == 'MediaResources':
            return self._load_media_resources(data, key)
            
        return self._load_table_bundles(data, key)
        
    def _load_asset_bundles(self, data: dict, key: str) -> list:
        items = data.get(key, [])
        return [{'name': item['Name'], 'size': item.get('Size', 0)} for item in items]
        
    def _load_media_resources(self, data: dict, key: str) -> list:
        table = data.get('Table', {})
        return [{'name': Path(item['Path']).name, 'size': item.get('Bytes', 0)} for item in table.values()]
        
    def _load_table_bundles(self, data: dict, key: str) -> list:
        table = data.get('Table', {})
        return [{'name': item['Name'], 'size': item.get('Size', 0)} for item in table.values()]

    def _create_table(self, items: list) -> dict:
        columns = [
            ("Category", "cyan", None, 15),
            ("Name", "green", None, None),
            ("Size", "yellow", "right", 10)
        ]

        total_items = len(items)
        self.visible_items = min(self.console.height - 8, total_items)
        
        self.selected_index = min(self.selected_index, total_items - 1)
        self.scroll_offset = min(self.scroll_offset, total_items - self.visible_items)
        self.scroll_offset = max(0, self.scroll_offset)

        start_idx = self.scroll_offset
        end_idx = min(start_idx + self.visible_items, total_items)
        
        rows = []
        for idx in range(start_idx, end_idx):
            category, name, size = items[idx]
            style = "reverse" if idx == self.selected_index else ""
            size_text = format_size(size)
            
            rows.append([
                Text(category, style=style),
                Text(name, style=style),
                Text(size_text, style=style)
            ])

        return create_table(columns, rows)

    def _filter_items(self) -> list:
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

    def _load_all_items(self) -> list:
        catalogs = self._load_catalogs()
        return [
            (category, item['name'], item['size'])
            for category, items in catalogs.items()
            for item in items
        ]

    def _handle_input(self, char: str, live: Live) -> bool:
        if not char:
            return True
            
        if char == '\x1b':  # ESC
            return False
            
        if char == '\x08' or char == '\x7f':  # Backspace
            self.query = self.query[:-1]
            self.selected_index = 0
            self.scroll_offset = 0
            
        elif char == '\x06':  # Ctrl+F
            filtered_items = self._filter_items()

            if filtered_items:
                current_category = filtered_items[self.selected_index][0]
                categories = sorted(set(item[0] for item in filtered_items))
                current_idx = categories.index(current_category)
                next_category = categories[(current_idx + 1) % len(categories)]
                
                for idx, (category, _, _) in enumerate(filtered_items):
                    if category == next_category:
                        self.selected_index = idx
                        if idx < self.scroll_offset or idx >= self.scroll_offset + self.visible_items:
                            self.scroll_offset = max(0, idx - self.visible_items // 2)
                        break

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
    
    def _download_selected_item(self, filtered_items: list, live: Live) -> None:
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
                Path(file.get('Path', '')).name or 
                Path(file['Url']).name
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

            get_character_input()  # Wait for key press
        
        live.start()

    def show(self):
        layout = create_layout()
        
        with Live(layout, console=self.console, screen=True, refresh_per_second=60) as live:
            for _ in iter(bool, True):
                filtered_items = self._filter_items()
                
                layout["search"].update(create_panel(
                    Text(f"> {self.query}", style="bold blue"),
                    title=f"[{len(filtered_items)} matches] [Press ESC to exit]",
                    style="blue"
                ))
                
                table = self._create_table(filtered_items)
                layout["content"].update(create_panel(table))
                
                if not self._handle_input(get_character_input(), live):
                    break
