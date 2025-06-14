import json
from pathlib import Path

from requests_cache import CachedSession
from rich.console import Console

from bacy import TableCatalog, MediaCatalog
from .. import __app_name__, __app_author__
from .fetcher import catalog_url
from ..helpers.json import load_json, save_json
from ..helpers.filemanager import get_cache_dir


class CatalogParser:
    def __init__(self):
        self.root = Path(__file__).parent.parent
        self.cache_dir = get_cache_dir(__app_name__, __app_author__)
        self.console = Console()

    @staticmethod
    def _fetch_bytes(catalog: str, file: str, cache: str) -> bytes:
        with CachedSession(cache, use_temp=True) as session:
            return session.get(f'{catalog}{file}').content

    def _fetch_table_bytes(self, catalog: str) -> bytes:
        return self._fetch_bytes(catalog, '/TableBundles/TableCatalog.bytes', 'tablebytes')

    def _fetch_media_bytes(self, catalog: str) -> bytes:
        return self._fetch_bytes(catalog, '/MediaResources/MediaCatalog.bytes', 'mediabytes')

    def _fetch_data(self, url: str, cache_name: str) -> dict:
        with CachedSession(cache_name=cache_name, use_temp=True) as session:
            try:
                return session.get(url).json()

            except (ConnectionError, TimeoutError) as e:
                self.console.log('[bold red]Error: Connection failed.[/bold red]')
                raise SystemExit(1) from e

    def fetch_catalog_url(self) -> str:
        server_api = catalog_url()
        server_data = self._fetch_data(server_api, 'serverapi')
        return server_data['ConnectionGroups'][0]['OverrideConnectionGroups'][-1]['AddressablesCatalogUrlRoot']

    def fetch_catalogs(self) -> None:
        server_url = self.fetch_catalog_url()

        self.console.print('[cyan]Fetching catalogs...[/cyan]')

        bundle_data = self._fetch_data(f'{server_url}/Android/bundleDownloadInfo.json', 'catalogurl')
        save_json(self.cache_dir / 'bundleDownloadInfo.json', bundle_data)

        table_data = self._fetch_table_bytes(catalog=server_url)
        table_catalog = TableCatalog.deserialize(table_data, server_url)
        save_json(self.cache_dir / 'TableCatalog.json', json.loads(table_catalog.to_json()))

        media_data = self._fetch_media_bytes(catalog=server_url)
        media_catalog = MediaCatalog.deserialize(media_data, server_url)
        save_json(self.cache_dir / 'MediaCatalog.json', json.loads(media_catalog.to_json()))

    def get_game_files(self) -> dict:
        server_url = self.fetch_catalog_url()

        bundle_data = load_json(self.cache_dir / 'bundleDownloadInfo.json')
        table_data = load_json(self.cache_dir / 'TableCatalog.json')
        media_data = load_json(self.cache_dir / 'MediaCatalog.json')

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
                    'url': f'{server_url}/TableBundles/{key}',
                    'crc': asset.get('crc', 0),
                    'size': asset.get('size', 0)
                }
                for key, asset in table_data['TableBundles'].items()
            ],
            'MediaResources': [
                {
                    'url': f'{server_url}/MediaResources/{value["path"]}',
                    'path': value['path'],
                    'crc': value.get('crc', 0),
                    'size': value.get('bytes', 0)
                }
                for key, value in media_data['MediaResources'].items()
            ],
        }

    def fetch_version(self) -> str:
        server_index = 'https://prod-noticeindex.bluearchiveyostar.com/prod/index.json'
        server_data = self._fetch_data(server_index, 'serverindex')
        return server_data['LatestClientVersion'] 