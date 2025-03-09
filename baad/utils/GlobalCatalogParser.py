from .CatalogParser import CatalogParser
from .GlobalCatalogFetcher import catalog_url


class GlobalCatalogParser(CatalogParser):
    def __init__(self, catalog_url: str | None = None):
        super().__init__(catalog_url)
        self.cache_dir = self.cache_dir / 'gl'
        self.cache_dir.mkdir(parents=True, exist_ok=True)

    def fetch_catalog_url(self) -> str:
        server_api = self.catalog_url if self.catalog_url else catalog_url()
        return server_api

    def fetch_resource_data(self) -> None:
        server_url = self.fetch_catalog_url()
        resource_data = self._fetch_data(f"{server_url}/resource-data.json", 'resourcedata')
        self.save_json(self.cache_dir / 'resource-data.json', resource_data)