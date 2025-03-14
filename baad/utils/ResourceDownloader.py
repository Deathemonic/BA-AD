import asyncio
from pathlib import Path

import aiofiles
from aiohttp import ClientError, ClientSession

from .ApkParser import ApkParser
from .CatalogParser import CatalogParser
from .Progress import create_live_display, create_progress_group
from .CatalogFilter import CatalogFilter


class ResourceDownloader:
    def __init__(self, update: bool = False, output: str | None = None, catalog_url: str | None = None, filter_pattern: str | None = None, version: str | None = None) -> None:
        self.root = Path(__file__).parent.parent
        self.output = output or Path.cwd() / 'output'
        self.update = update
        self.catalog_url = catalog_url
        self.filter_pattern = filter_pattern
        self.version = version

        self.semaphore = None
        self.catalog_parser = CatalogParser(catalog_url)
        self.categories = {
            'asset': 'AssetBundles',
            'table': 'TableBundles',
            'media': 'MediaResources',
        }
        self.live = create_live_display()
        self.progress_group, self.download_progress, _, _, self.console = create_progress_group()

    @staticmethod
    def _get_file_path(file: dict) -> str | Path:
        if 'path' in file:
            return file['path']

        elif isinstance(file['url'], str):
            return Path(file['url']).name

        return Path(file['url'])

    async def _check_existing_file(self, file_path: Path, crc: int) -> bool:
        if not file_path.exists():
            return False

        if self.catalog_parser._calculate_crc32(file_path) != crc:
            return False

        self.console.print(f'[green]Skipping {file_path.name}, already downloaded.[/green]')
        return True

    async def _download_file_content(
        self, session: ClientSession, url: str, fp: Path, size: int, retries: int = 3
    ) -> bool:
        for attempt in range(retries):
            bytes_downloaded = 0

            try:
                async with session.get(url) as response:
                    async with aiofiles.open(fp, 'wb') as f:
                        async for chunk in response.content.iter_chunked(8192):
                            if not chunk:
                                break

                            await f.write(chunk)
                            bytes_downloaded += len(chunk)

                if bytes_downloaded == size:
                    self.console.print(f'[green]Successfully downloaded {fp.name}[/green]')
                    return True

            except Exception as e:
                self.console.log(f'[yellow]Error downloading {fp.name} {str(e)}[/yellow]')
                continue

            if attempt < retries - 1:
                await asyncio.sleep(2**attempt)

        self.console.log(f'[bold red]Failed to download {fp.name} after {retries} attempts.[/bold red]')
        return False

    async def _verify_download(self, file_path: Path, crc: int) -> bool:
        if self.catalog_parser._calculate_crc32(file_path) != crc:
            self.console.log(f'[yellow]Hash mismatch for {file_path.name}, retrying...[/yellow]')
            file_path.unlink(missing_ok=True)
            return False

        return True

    async def _download_file(
        self, session: ClientSession, url: str, file_path: Path, crc: int, retries: int = 3
    ) -> None:
        if await self._check_existing_file(file_path, crc):
            return

        file_path.parent.mkdir(parents=True, exist_ok=True)
        total_size = await self.get_file_size(session, url)

        if total_size is None:
            self.console.log(f'[bold red]Failed to get file size for {url}[/bold red]')
            return

        download_success = await self._download_file_content(session, url, file_path, total_size, retries)

        if download_success and await self._verify_download(file_path, crc):
            return

    async def _download_category(self, files: list, base_path: Path) -> None:
        async with ClientSession() as session:
            tasks = [
                self._download_file(
                    session,
                    file['url'],
                    base_path / self._get_file_path(file),
                    file['crc'],
                )
                for file in files
            ]
            await asyncio.gather(*tasks)

    async def _download_all_categories(self, game_files: dict, categories: list) -> None:
        for category, files in game_files.items():
            if category in categories:
                await self._download_category(files, Path(self.output) / category)

    async def get_file_size(self, session: ClientSession, url: str) -> int | None:
        try:
            async with session.head(url, allow_redirects=True) as response:
                return int(response.headers.get('Content-Length', 0))

        except (ClientError, ValueError):
            return None

    def initialize_download(self) -> dict:
        self.fetch_catalog_url()
        self.catalog_parser.fetch_catalogs()
        result = self.catalog_parser.get_game_files()
        
        game_files_dir = self.catalog_parser.cache_dir / (self.version or self.catalog_parser.fetch_version())
        game_files_dir.mkdir(parents=True, exist_ok=True)
        game_files_path = game_files_dir / 'GameFiles.json'
        self.catalog_parser.save_json(game_files_path, result)
        
        if not self.filter_pattern:
            return result
            
        catalog_filter = CatalogFilter(game_files_path)
        filtered_result = catalog_filter.filter_files(self.filter_pattern)
        
        if not any(filtered_result.values()):
            self.console.print(f"[yellow]No files found matching filter pattern: {self.filter_pattern}[/yellow]")
            return {}
        
        return filtered_result

    def download(self, assets: bool = True, tables: bool = True, media: bool = True, limit: int | None = 5) -> None:
        game_files = self.initialize_download()

        categories = [
            self.categories[cat] for cat, enabled in [('asset', assets), ('table', tables), ('media', media)] if enabled
        ]

        self.semaphore = asyncio.Semaphore(limit if limit is not None else float('inf'))
        asyncio.run(self._download_all_categories(game_files, categories))

    def fetch_catalog_url(self) -> None:
        if self.catalog_url:
            self.console.print(f'[cyan]Using provided catalog URL: {self.catalog_url}[/cyan]')
            return

        ApkParser(version=self.version).download_apk(self.update)
        self.console.print('[cyan]Fetching catalog URL...[/cyan]')
        catalog_url = self.catalog_parser.fetch_catalog_url()
        self.console.print(f'[green]Catalog URL fetched: {catalog_url}[/green]')
