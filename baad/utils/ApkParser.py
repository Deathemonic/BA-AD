from pathlib import Path
from zipfile import ZipFile

import cloudscraper
import requests
import shutil
from platformdirs import user_data_dir

from .Progress import create_live_display, create_progress_group
from .. import __app_name__, __app_author__
from .CatalogParser import CatalogParser


class ApkParser:
    def __init__(self, apk_url: str | None = None, apk_path: str | None = None, version: str | None = None) -> None:
        self.version = version
        self.catalog_parser = CatalogParser()

        self.root = Path(__file__).parent.parent
        self.cache_dir = Path(user_data_dir(__app_name__, __app_author__)) / 'jp'
        self.cache_dir.mkdir(parents=True, exist_ok=True)

        if not self.version:
            self.version = self.catalog_parser.fetch_version()
        
        self.version_dir = self.cache_dir / self.version
        self.version_dir.mkdir(parents=True, exist_ok=True)
        
        self.apk_path = apk_path or self.version_dir / 'BlueArchive.xapk'
        self.apk_url = apk_url or self._get_apk_url()

        self.live = create_live_display()
        self.progress_group, self.download_progress, self.extract_progress, self.print_progress, self.console = (
            create_progress_group()
        )

        self.scraper = cloudscraper.create_scraper()

    def _get_apk_url(self) -> str:
        return f'https://d.apkpure.com/b/XAPK/com.YostarJP.BlueArchive?version={self.version}'

    @staticmethod
    def _get_files(zip: ZipFile) -> set:
        return {file_info for file_info in zip.infolist() if not file_info.is_dir()}

    def _get_response(self) -> requests.Response | SystemExit:
        try:
            return self.scraper.get(self.apk_url, stream=True)

        except (ConnectionError, TimeoutError, requests.exceptions.RequestException) as e:
            self.console.log(f'[bold red]Error: Connection Failed{str(e)}[/bold red]')
            self.console.log(f'[bold red]{str(e)}[/bold red]')
            raise SystemExit(1) from e

    def _download_file(self, response: requests.Response) -> None:
        total_size = int(response.headers.get('content-length', 0))
        download_task = self.download_progress.add_task('[red]Downloading APK...', total=total_size)

        apk_path = Path(self.apk_path)
        apk_path.parent.mkdir(parents=True, exist_ok=True)

        with self.live:
            with open(apk_path, 'wb') as f:
                for chunk in response.iter_content(chunk_size=8192):
                    if chunk:
                        f.write(chunk)

                    self.download_progress.update(download_task, advance=len(chunk))
                    self.live.update(self.progress_group)

            self.download_progress.update(download_task, description='[green]APK downloaded...')
            self.live.update(self.progress_group)

    def _force_download(self) -> None:
        response = self._get_response()
        if isinstance(response, requests.Response):
            self._delete_outdated_files()
            self._download_file(response)

    def _delete_outdated_files(self) -> None:
        xapk_path = Path(self.apk_path)
        apk_folder = xapk_path.parent / 'apk'
        data_folder = xapk_path.parent / 'data'

        for folder in [apk_folder, data_folder]:
            if folder.exists():
                shutil.rmtree(folder)
                self.console.print(f"[yellow]Deleted outdated folder: {folder}[/yellow]")

    def _fetch_size(self) -> int:
        try:
            response = self.scraper.get(self.apk_url, stream=True, timeout=60)
            return int(response.headers.get('content-length', 0))

        except (ConnectionError, TimeoutError, requests.exceptions.RequestException) as e:
            self.console.log(f'[bold red]Error: {str(e)}[/bold red]')
            raise SystemExit(1) from e

    def _log_size(self, local: int, remote: int) -> None:
        if local == remote:
            self.console.print(f'[green]APK version {self.version} is up to date. Skipping download...[/green]')

        if local < remote:
            self.console.print(f'[yellow]APK version {self.version} is out of date. Downloading...[/yellow]')

    def _parse_zipfile(self, apk_path: Path, extract_path: Path) -> None:
        with ZipFile(apk_path, 'r') as zip:
            extract = self._get_files(zip)
            self._extract_files(zip, extract, extract_path)

    def _extract_files(self, zip: ZipFile, extract: set, extract_path: Path) -> None:
        extract_task = self.extract_progress.add_task('[green]Extracting...', total=len(extract))

        with self.live:
            for file_info in extract:
                target_path = Path(self.apk_path).parent / 'data' / Path(file_info.filename)
                target_path.parent.mkdir(parents=True, exist_ok=True)

                zip.extract(file_info, extract_path)

                self.extract_progress.update(extract_task, advance=1)
                self.live.update(self.progress_group)

            self.extract_progress.update(extract_task, description='[green]APK Extracted...')
            self.live.update(self.progress_group)

    def compare_apk(self) -> bool:
        remote_size = self._fetch_size()
        if remote_size is None:
            return False

        local_size = Path(self.apk_path).stat().st_size
        self._log_size(local_size, remote_size)

        return local_size < remote_size

    def download_apk(self, update: bool = False) -> None:
        if update or not Path(self.apk_path).exists():
            self._force_download()
            self.extract_apk()
            return

        if self.compare_apk():
            self._force_download()
            return

        self.extract_apk()

    def extract_apk(self) -> None:
        xapk_path = Path(self.apk_path)
        apk_path = xapk_path.parent / 'apk'
        data_path = xapk_path.parent / 'data'
        unity_apk = apk_path / 'UnityDataAssetPack.apk'
        main_apk = apk_path / 'com.YostarJP.BlueArchive.apk'

        self._parse_zipfile(xapk_path, apk_path)
        self._parse_zipfile(unity_apk, data_path)
        self._parse_zipfile(main_apk, data_path)
