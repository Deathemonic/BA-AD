import os
import platform
import subprocess
import zipfile
from pathlib import Path

import requests

from ..helpers.progress import create_live_display, create_progress_group
from ..helpers.filemanager import ensure_directory_exists, get_asset_output_dir, get_extracted_dir

# TODO: Deprecated (Moved to BA-AE)
class AssetStudioExtractor:
    def __init__(self, output_path: Path) -> None:
        self.asset_path = get_asset_output_dir(output_path)
        self.extracted_path = get_extracted_dir(output_path, 'Asset')

        self.root = self.root = Path(__file__).parent.parent

        self.bin_path = self.root / 'public' / 'bin'
        self.asset_studio_url = 'https://github.com/aelurum/AssetStudio/releases/latest/download/'
        self.asset_studio_path = self._get_asset_studio_path()

        self.types = {'Texture2D', 'Sprite', 'TextAsset', 'MonoBehaviour', 'Font', 'Shader', 'AudioClip', 'Mesh'}

        self.live = create_live_display()
        self.progress_group, _, _, _, self.console = create_progress_group()

    def _get_asset_studio_path(self) -> Path:
        if platform.system() == 'Windows':
            return self.bin_path / 'AssetStudioModCLI.exe'
        return self.bin_path / 'AssetStudioModCLI'

    def _get_asset_studio_url(self) -> str:
        system = platform.system()
        if system == 'Windows':
            return self.asset_studio_url + 'AssetStudioModCLI_net8_portable.zip'
        elif system == 'macOS':
            return self.asset_studio_url + 'AssetStudioModCLI_net6_mac64.zip'
        return self.asset_studio_url + 'AssetStudioModCLI_net6_linux64.zip'

    def _download_asset_studio(self) -> None:
        if self.asset_studio_path.exists():
            self.console.print('[cyan]AssetStudioModCLI already exists. Skipping...[/cyan]')
            return

        zip_path = self.bin_path / 'AssetStudioModCLI.zip'
        ensure_directory_exists(self.bin_path)

        if not zip_path.exists():
            self.console.print('[cyan]Downloading AssetStudioModCLI...[/cyan]')

            response = requests.get(self._get_asset_studio_url())
            zip_path.write_bytes(response.content)

        if not self.asset_studio_path.exists():
            self.console.print('[cyan]Extracting AssetStudioModCLI...[/cyan]')

            with zipfile.ZipFile(zip_path, 'r') as zip_ref:
                zip_ref.extractall(self.bin_path)

            if platform.system() == 'Windows':
                os.chmod(self.asset_studio_path, 0o755)

    def _run_asset_studio_command(self, command: list) -> subprocess.CompletedProcess:
        try:
            result = subprocess.run(command, check=True, text=True)
            return result

        except subprocess.CalledProcessError as e:
            self.console.log(f'[red]Error running AssetStudio: {e}[/red]')
            self.console.log(f'[red]Error output: {e.stderr}[/red]')
            raise

    def _extract_bundle(self, bundle_path: Path) -> None:
        bundle_name = bundle_path.stem
        bundle_output = self.extracted_path / bundle_name
        ensure_directory_exists(bundle_output)

        command = [
            str(self.asset_studio_path),
            str(bundle_path),
            '-o',
            str(bundle_output),
            '-t',
            ','.join(self.types),
            '--image-format',
            'tga',
        ]

        self.console.print(f'[cyan]Extracting regular assets from {bundle_name}...[/cyan]')
        self._run_asset_studio_command(command)

        fbx_output = bundle_output / 'FBX'
        ensure_directory_exists(fbx_output)

        command = [
            str(self.asset_studio_path),
            str(bundle_path),
            '-m',
            'splitObjects',
            '-o',
            str(fbx_output),
            '--image-format',
            'tga',
        ]

        self.console.print(f'[cyan]Extracting FBX objects from {bundle_name}...[/cyan]')
        self._run_asset_studio_command(command)

    def extract_assets(self) -> None:
        try:
            with self.live:
                self._download_asset_studio()

                bundle_files = list(Path(self.asset_path).glob('*.bundle'))
                for bundle_file in bundle_files:
                    self._extract_bundle(bundle_file)
                self.console.print('[green]Asset extraction completed![/green]')

        finally:
            self.live.stop()
