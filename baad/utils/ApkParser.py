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

        self.progress_group, self.download_progress, self.extract_progress, self.print_progress, self.console = (
            create_progress_group()
        )
        self.scraper = cloudscraper.create_scraper()

        if not self.version:
            self.version = self.catalog_parser.fetch_version()
        
        self.version_dir = self.cache_dir / self.version
        self.version_dir.mkdir(parents=True, exist_ok=True)
        self.apk_path = apk_path or self.version_dir / 'BlueArchive.xapk'
        
        self.apk_url = apk_url or self._get_apk_url()
        self.live = create_live_display()


    def _get_apk_url(self) -> str:
        base_url = "https://d.apkpure.com/b"
        app_id = "com.YostarJP.BlueArchive"
        
        if not self.version:
            return f'{base_url}/XAPK/{app_id}?version=latest'
        
        version_code = self._extract_version_code(self.version)
        
        urls_to_try = self._build_url_list(base_url, app_id, self.version, version_code)
        
        for url in urls_to_try:
            if valid_url := self._check_url_validity(url):
                return valid_url

        self.console.print(f"[yellow]Could not determine format for version {self.version}, using latest[/yellow]")
        return f'{base_url}/XAPK/{app_id}?version=latest'

    def _extract_version_code(self, version: str) -> str | None:
        if not version.startswith("1."):
            return None
        
        parts = version.split('.')
        if len(parts) >= 3 and parts[2] != "0":
            return parts[2]
        
        return None

    def _build_url_list(self, base_url: str, app_id: str, version: str, version_code: str | None) -> list[str]:
        versioncode_urls = [
            f'{base_url}/APK/{app_id}?versionCode={version_code}',
            f'{base_url}/XAPK/{app_id}?versionCode={version_code}'
        ] if version_code else []
        
        version_urls = [
            f'{base_url}/APK/{app_id}?version={version}',
            f'{base_url}/XAPK/{app_id}?version={version}'
        ]
        
        return versioncode_urls + version_urls

    def _check_url_validity(self, url: str) -> str | None:
        try:
            response = self.scraper.get(url, stream=True, timeout=10)
            content_start = next(response.iter_content(256), b'')
            
            html_markers = [b'<!DOCTYPE', b'<html', b'<HTML']
            is_html = any(marker in content_start for marker in html_markers)
            
            if not is_html:
                url_type = "APK" if "/APK/" in url else "XAPK"
                param_type = "versionCode" if "versionCode" in url else "version"
                self.console.print(f"[cyan]Using {url_type} format with {param_type} for version {self.version}[/cyan]")
                return url
            
            url_type = "APK" if "/APK/" in url else "XAPK"
            param_type = "versionCode" if "versionCode" in url else "version"
            self.console.print(f"[yellow]{url_type} URL with {param_type} returned HTML, trying next option...[/yellow]")
            
        except Exception as e:
            self.console.print(f"[yellow]Error checking URL {url}: {str(e)}[/yellow]")
        
        return None

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

    def _extract_regular_apk(self, file_path: Path, data_folder: Path) -> None:
        self.console.print("[cyan]Detected regular APK format. Extracting...[/cyan]")
        self._parse_zipfile(file_path, data_folder)

    def _extract_xapk(self, file_path: Path, apk_folder: Path, data_folder: Path) -> None:
        self.console.print("[cyan]Detected XAPK format. Extracting...[/cyan]")
        
        self._parse_zipfile(file_path, apk_folder)
        
        apk_files = {
            'unity': apk_folder / 'UnityDataAssetPack.apk',
            'main': apk_folder / 'com.YostarJP.BlueArchive.apk'
        }
        
        for apk_type, apk_path in apk_files.items():
            if apk_path.exists():
                self._parse_zipfile(apk_path, data_folder)
                continue
            
            self.console.print(f"[yellow]Warning: {apk_path.name} not found in XAPK[/yellow]")

    def _is_regular_apk(self, file_path: Path) -> bool:
        try:
            with ZipFile(file_path, 'r') as zip_file:
                file_list = zip_file.namelist()
                
                xapk_indicators = ['manifest.json', 'com.YostarJP.BlueArchive.apk', 'UnityDataAssetPack.apk']
                if any(indicator in file_list for indicator in xapk_indicators):
                    return False
                
                apk_indicators = ['AndroidManifest.xml', 'classes.dex', 'resources.arsc']
                has_apk_indicators = any(
                    any(entry.endswith(indicator) for entry in file_list) 
                    for indicator in apk_indicators
                )
                
                return has_apk_indicators or file_path.suffix.lower() == '.apk'
                
        except Exception as e:
            self.console.print(f"[yellow]Warning: Error checking APK format: {str(e)}[/yellow]")
            return False

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
        file_path = Path(self.apk_path)
        apk_folder = file_path.parent / 'apk'
        data_folder = file_path.parent / 'data'
        
        data_folder.mkdir(parents=True, exist_ok=True)
        
        if self._is_regular_apk(file_path):
            self._extract_regular_apk(file_path, data_folder)
            return
            
        self._extract_xapk(file_path, apk_folder, data_folder)

