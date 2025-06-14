from pathlib import Path
import shutil
from zipfile import ZipFile
from platformdirs import user_data_dir, user_cache_dir


def ensure_directory_exists(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def delete_directory(directory: Path) -> bool:
    if directory.exists():
        shutil.rmtree(directory)
        return True
    return False


def extract_files_from_zip(zip_file: Path, extract_path: Path, file_infos: list = None) -> None:
    with ZipFile(zip_file, 'r') as zip:
        if file_infos is None:
            file_infos = [file for file in zip.infolist() if not file.is_dir()]
        
        for file_info in file_infos:
            target_path = extract_path / Path(file_info.filename)
            target_path.parent.mkdir(parents=True, exist_ok=True)
            
            zip.extract(file_info, extract_path)


def get_zip_file_infos(zip_file: Path) -> list:
    with ZipFile(zip_file, 'r') as zip:
        return [file_info for file_info in zip.infolist() if not file_info.is_dir()]


def get_data_dir(app_name: str, app_author: str, version: str = "jp") -> Path:
    data_dir = Path(user_data_dir(app_name, app_author)) / version
    ensure_directory_exists(data_dir)
    return data_dir


def get_cache_dir(app_name: str, app_author: str, version: str = "jp") -> Path:
    cache_dir = Path(user_cache_dir(app_name, app_author)) / version
    ensure_directory_exists(cache_dir)
    return cache_dir


def get_output_dir(base_path: Path = None) -> Path:
    output_dir = base_path or Path.cwd() / 'output'
    ensure_directory_exists(output_dir)
    return output_dir


def get_asset_output_dir(base_path: Path = None) -> Path:
    output_dir = get_output_dir(base_path) / 'AssetBundles'
    ensure_directory_exists(output_dir)
    return output_dir


def get_table_output_dir(base_path: Path = None) -> Path:
    output_dir = get_output_dir(base_path) / 'TableBundles'
    ensure_directory_exists(output_dir)
    return output_dir


def get_media_output_dir(base_path: Path = None) -> Path:
    output_dir = get_output_dir(base_path) / 'MediaResources'
    ensure_directory_exists(output_dir)
    return output_dir


def get_extracted_dir(base_path: Path = None, subfolder: str = None) -> Path:
    path = get_output_dir(base_path)
    
    if subfolder:
        path = path / f"{subfolder}Extracted"
    else:
        path = path / "Extracted"
        
    ensure_directory_exists(path)
    return path
