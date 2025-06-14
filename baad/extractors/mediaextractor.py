import asyncio
from pathlib import Path
from zipfile import BadZipFile

from bacy import TableZipFile
from ..helpers.progress import create_live_display, create_progress_group
from ..helpers.filemanager import ensure_directory_exists, get_media_output_dir, get_extracted_dir


# TODO: Deprecated (Moved to BA-AE)
class MediaExtractor:
    def __init__(self, output: str) -> None:
        self.media_path = get_media_output_dir(output) / 'GameData' / 'Audio' / 'VOC_JP'
        self.extracted_path = get_extracted_dir(output, 'Media') / 'GameData' / 'Audio' / 'VOC_JP'
        ensure_directory_exists(self.extracted_path)

        self.live = create_live_display()
        self.progress_group, _, self.extract_progress, self.print_progress, self.console = create_progress_group()

    async def extract_media(self, media_file: Path | str, extract_task: int) -> None:
        try:
            with open(media_file, 'rb') as f:
                zip_bytes = f.read()
            
            zip_file = TableZipFile(zip_bytes, media_file.name.lower())
            media_dir_fp = self.extracted_path / media_file.stem
            ensure_directory_exists(media_dir_fp)

            self.print_progress.add_task(f"[cyan]Extracting {media_file.name}...[/cyan]")

            try:
                for name, data in zip_file.extract_all():
                    fp = media_dir_fp / name
                    ensure_directory_exists(fp.parent)
                    fp.write_bytes(data)
                
                self.extract_progress.update(extract_task, advance=1)
                self.live.update(self.progress_group)

            except Exception as e:
                self.print_progress.add_task(f'[red]Error extracting {media_file.name}: {str(e)}[/red]')

        except BadZipFile:
            self.print_progress.add_task(f'[red]Error: {media_file} is not a valid zip file.[/red]')
        except Exception as e:
            self.print_progress.add_task(f'[red]Error reading {media_file}: {str(e)}[/red]')

    async def extract_all_media(self) -> None:
        media_files = list(Path(self.media_path).glob('*.zip'))
        if not media_files:
            self.print_progress.add_task("[yellow]No media files found to extract[/yellow]")
            return

        self.print_progress.add_task(f"[cyan]Found {len(media_files)} media archives to extract[/cyan]")
        
        extract_task = self.extract_progress.add_task('[green]Extracting Media...', total=len(media_files))
        
        tasks = [self.extract_media(media_file, extract_task) for media_file in media_files]
        await asyncio.gather(*tasks)
        
        self.extract_progress.update(extract_task, description='[green]Media Extracted...')
        self.print_progress.add_task("[green]All files have been extracted successfully![/green]")

    def run_extraction(self) -> None:
        try:
            with self.live:
                asyncio.run(self.extract_all_media())
        finally:
            self.live.stop() 