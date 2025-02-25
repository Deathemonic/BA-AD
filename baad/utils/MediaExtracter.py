import asyncio
from pathlib import Path
from zipfile import BadZipFile

from rich.console import Console
from ..lib.TableService import TableZipFile


class MediaExtracter:
    def __init__(self, output: str) -> None:
        self.media_path = output or Path.cwd() / 'output' / 'MediaResources' / 'GameData' / 'Audio' / 'VOC_JP'
        self.extracted_path = Path(self.media_path).parent.parent.parent.parent / 'MediaExtracted' / 'GameData' / 'Audio' / 'VOC_JP'
        self.console = Console()

    async def extract_media(self, media_file: Path | str) -> None:
        try:
            with TableZipFile(media_file) as tz:
                file_list = tz.namelist()
                media_dir_fp = self.extracted_path / media_file.stem
                media_dir_fp.mkdir(parents=True, exist_ok=True)

                self.console.print(f"[cyan]Extracting {media_file.name}...[/cyan]")

                for name in file_list:
                    data = tz.read(name)
                    fp = media_dir_fp / name
                    fp.parent.mkdir(parents=True, exist_ok=True)
                    fp.write_bytes(data)
                    self.console.print(f"[green]  Extracted: {name}[/green]")

        except BadZipFile:
            self.console.print(f'[red]Error: {media_file} is not a valid zip file.[/red]')
            
        except RuntimeError as e:
            self.console.print(f'[red]Error extracting {media_file}: {str(e)}[/red]')

    async def extract_all_media(self) -> None:
        media_files = list(Path(self.media_path).glob('*.zip'))
        if not media_files:
            self.console.print("[yellow]No media files found to extract[/yellow]")
            return

        self.console.print(f"[cyan]Found {len(media_files)} media archives to extract[/cyan]")
        
        tasks = [self.extract_media(media_file) for media_file in media_files]
        await asyncio.gather(*tasks)
        
        self.console.print("[green]All files have been extracted successfully![/green]")

    def run_extraction(self) -> None:
        asyncio.run(self.extract_all_media()) 