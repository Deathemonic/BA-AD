from pathlib import Path

import cloudscraper
import requests

from ..helpers.progress import create_live_display, create_progress_group
from ..helpers.filemanager import (
    ensure_directory_exists,
    delete_directory,
    get_zip_file_infos,
    extract_files_from_zip,
    get_data_dir
)
from .. import __app_name__, __app_author__


class Apk:
    def __init__(self, apk_url: str | None = None, apk_path: str | None = None) -> None:
        self.apk_url = apk_url or 'https://d.apkpure.com/b/XAPK/com.YostarJP.BlueArchive?version=latest'

        self.root = Path(__file__).parent.parent
        self.cache_dir = get_data_dir(__app_name__, __app_author__)
        self.apk_path = apk_path or self.cache_dir / 'BlueArchive.xapk'

        self.live = create_live_display()
        self.progress_group, self.download_progress, self.extract_progress, self.print_progress, self.console = (
            create_progress_group()
        )

        self.scraper = cloudscraper.create_scraper()

    def apk_exists(self) -> bool:
        return Path(self.apk_path).exists()

    def is_outdated(self) -> bool:
        if not self.apk_exists():
            return True
            
        remote_size = self._fetch_size()
        if remote_size is None:
            return True

        local_size = Path(self.apk_path).stat().st_size
        return local_size < remote_size

    def _fetch_size(self) -> int | None:
        try:
            response = self.scraper.get(self.apk_url, stream=True)
            return int(response.headers.get('content-length', 0))
        except (ConnectionError, TimeoutError, requests.exceptions.RequestException) as e:
            self.console.log(f'[bold red]Error: {str(e)}[/bold red]')
            return None

    def _get_response(self) -> requests.Response | SystemExit:
        try:
            return self.scraper.get(self.apk_url, stream=True)
        except (ConnectionError, TimeoutError, requests.exceptions.RequestException) as e:
            self.console.log(f'[bold red]Error: Connection Failed{str(e)}[/bold red]')
            raise SystemExit(1) from e

    def _download_file(self, response: requests.Response) -> None:
        total_size = int(response.headers.get('content-length', 0))
        download_task = self.download_progress.add_task('[red]Downloading APK...', total=total_size)

        apk_path = Path(self.apk_path)
        ensure_directory_exists(apk_path.parent)

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
            if delete_directory(folder):
                self.console.print(f"[yellow]Deleted outdated folder: {folder}[/yellow]")

    def _parse_zipfile(self, apk_path: Path, extract_path: Path) -> None:
        file_infos = get_zip_file_infos(apk_path)
        self._extract_files(apk_path, file_infos, extract_path)

    def _extract_files(self, zip_path: Path, file_infos: list, extract_path: Path) -> None:
        extract_task = self.extract_progress.add_task('[green]Extracting...', total=len(file_infos))

        with self.live:
            for file_info in file_infos:
                extract_files_from_zip(zip_path, extract_path, [file_info])
                
                self.extract_progress.update(extract_task, advance=1)
                self.live.update(self.progress_group)

            self.extract_progress.update(extract_task, description='[green]APK Extracted...')
            self.live.update(self.progress_group)

    def download_apk(self, update: bool = False) -> None:
        if update or not self.apk_exists():
            self._force_download()
            self.extract_apk()
            return

        if self.is_outdated():
            self._force_download()
            return

        self.extract_apk()

    def extract_apk(self) -> None:
        xapk_path = Path(self.apk_path)
        apk_path = self.cache_dir / 'apk'
        data_path = self.cache_dir / 'data'
        unity_apk = apk_path / 'UnityDataAssetPack.apk'
        main_apk = apk_path / 'com.YostarJP.BlueArchive.apk'

        self._parse_zipfile(xapk_path, apk_path)
        self._parse_zipfile(unity_apk, data_path)
        self._parse_zipfile(main_apk, data_path)

