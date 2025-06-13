import binascii
import json
from pathlib import Path

from requests_cache import CachedSession
from rich.console import Console
from platformdirs import user_cache_dir

from .. import __app_name__, __app_author__
from ..lib.CatalogDecrypter import CatalogDecrypter
from .CatalogFetcher import catalog_url


class CatalogParser:
    def __init__(self, catalog_url: str | None = None, version: str | None = None):
        self.root = Path(__file__).parent.parent
        self.cache_dir = Path(user_cache_dir(__app_name__, __app_author__)) / 'jp'
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.console = Console()
        self.catalog_url = catalog_url or None
        self.version = version

    @staticmethod
    def _calculate_crc32(file_path: Path) -> int:
        with open(file_path, 'rb') as f:
            return binascii.crc32(f.read()) & 0xFFFFFFFF

    @staticmethod
    def _fetch_bytes(catalog: str, file: str, cache: str) -> bytes:
        with CachedSession(cache, use_temp=True) as session:
            response = session.get(f'{catalog}{file}')
            if response.status_code != 200:
                raise Exception(f"HTTP error {response.status_code}: {response.reason} for URL {catalog}{file}")
            return response.content

    @staticmethod
    def _load_json(file_path: Path) -> dict:
        with open(file_path, 'r') as f:
            return json.load(f)

    @staticmethod
    def save_json(file_path: Path, data: dict) -> None:
        with open(file_path, 'w') as f:
            json.dump(data, f, indent=4)

    def _fetch_table_bytes(self, catalog: str) -> bytes:
        return self._fetch_bytes(catalog, '/TableBundles/TableCatalog.bytes', 'tablebytes')

    def _fetch_media_bytes(self, catalog: str) -> bytes:
        paths = [
            '/MediaResources/Catalog/MediaCatalog.bytes',
            '/MediaResources/MediaCatalog.bytes'
        ]

        for path in paths:
            try:
                return self._fetch_bytes(catalog, path, 'mediabytes')
            except Exception:
                continue
                
        raise Exception("Failed to fetch media bytes from all available paths")

    def _fetch_data(self, url: str, cache_name: str) -> dict:
        with CachedSession(cache_name=cache_name, use_temp=True) as session:
            try:
                return session.get(url).json()

            except (ConnectionError, TimeoutError) as e:
                self.console.log('[bold red]Error: Connection failed.[/bold red]')
                raise SystemExit(1) from e

    def fetch_catalog_url(self) -> str:
        if not self.catalog_url:
            server_api = catalog_url(self.version)
            server_data = self._fetch_data(server_api, 'serverapi')
            return server_data['ConnectionGroups'][0]['OverrideConnectionGroups'][-1]['AddressablesCatalogUrlRoot']
        
        if self.catalog_url.startswith(('http://', 'https://')):
            return self.catalog_url
        
        if '_' in self.catalog_url:
            return f'https://prod-clientpatch.bluearchiveyostar.com/{self.catalog_url}'
        
        return f'https://{self.catalog_url}'

    def fetch_catalogs(self) -> None:
        server_url = self.fetch_catalog_url()

        self.console.print('[cyan]Fetching catalogs...[/cyan]')

        bundle_data = self._fetch_data(f'{server_url}/Android/bundleDownloadInfo.json', 'catalogurl')
        self.save_json(self.cache_dir / 'bundleDownloadInfo.json', bundle_data)

        table_data = self._fetch_table_bytes(catalog=server_url)
        table_catalog = CatalogDecrypter.from_bytes(table_data, server_url, media=False)
        table_catalog.to_json(self.cache_dir / 'TableCatalog.json', media=False)

        media_data = self._fetch_media_bytes(catalog=server_url)
        media_catalog = CatalogDecrypter.from_bytes(media_data, server_url, media=True)
        media_catalog.to_json(self.cache_dir / 'MediaCatalog.json', media=True)

    def get_game_files(self) -> dict:
        server_url = self.fetch_catalog_url()

        bundle_data = self._load_json(self.cache_dir / 'bundleDownloadInfo.json')
        table_data = self._load_json(self.cache_dir / 'TableCatalog.json')
        media_data = self._load_json(self.cache_dir / 'MediaCatalog.json')

        return {
            'AssetBundles': [
                {
                    'url': f'{server_url}/Android/{asset["Name"]}',
                    'crc': asset.get('Crc', 0),
                    'size': asset.get('Size', 0)
                }
                for asset in bundle_data['BundleFiles']
            ],
            'TableBundles': [
                {
                    'url': f'{server_url}/TableBundles/{asset["name"]}',
                    'crc': asset.get('crc', 0),
                    'size': asset.get('size', 0)
                }
                for key, asset in table_data['TableBundles'].items()
            ],
            'MediaResources': [
                {
                    'url': f'{server_url}/MediaResources/{value["path"]}'.replace("\\", "/"),
                    'path': value['path'].replace("\\", "/"),
                    'crc': value.get('crc', 0),
                    'bytes': value.get('bytes', 0)
                }
                for key, value in media_data['MediaResources'].items()
            ],
        }

    def fetch_version(self) -> str:
        server_index = 'https://prod-noticeindex.bluearchiveyostar.com/prod/index.json'
        server_data = self._fetch_data(server_index, 'serverindex')
        return server_data['LatestClientVersion']
