import json
from base64 import b64encode
from pathlib import Path
from platformdirs import user_data_dir

from ..lib.TableEncryptionService import TableEncryptionService
from .. import __app_name__, __app_author__


def _search_for_pattern(path: Path, pattern: bytes) -> bytes | None:
    if not path.exists():
        return None
        
    for config_file in path.rglob('*'):
        if not config_file.is_file():
            continue
            
        try:
            content = config_file.read_bytes()

            if pattern in content:
                start_index = content.index(pattern)
                data = content[start_index + len(pattern):]
                return data[:-2]
            
        except Exception as e:
            print(f"Error reading file {config_file}: {e}")
    
    return None


def _get_cache_paths(cache_base: Path, existing_paths: list) -> list:
    if not cache_base.exists():
        return []
        
    cache_paths = []
    for version_dir in cache_base.iterdir():
        if not version_dir.is_dir():
            continue
            
        cache_path = version_dir / 'data' / 'assets' / 'bin' / 'Data'
        if not cache_path.exists():
            continue
            
        if cache_path in [p[1] for p in existing_paths]:
            continue
            
        cache_paths.append((f"Cache ({version_dir.name})", cache_path))
    
    return cache_paths


def find_game_config(version: str = None) -> None | bytes:
    pattern = bytes([
        0x47, 0x61, 0x6D, 0x65, 0x4D, 0x61, 0x69, 0x6E, 0x43, 0x6F, 0x6E, 0x66,
        0x69, 0x67, 0x00, 0x00, 0x92, 0x03, 0x00, 0x00,
    ])
    
    search_paths = []
    
    if version:
        cache_dir = Path(user_data_dir(__app_name__, __app_author__)) / 'jp' / version
        version_path = cache_dir / 'data' / 'assets' / 'bin' / 'Data'
        search_paths.append(("Version-specific", version_path))
    
    default_path = Path(__file__).parent.parent / 'public' / 'jp' / 'data' / 'assets' / 'bin' / 'Data'
    search_paths.append(("Default", default_path))
    
    cache_base = Path(user_data_dir(__app_name__, __app_author__)) / 'jp'
    cache_paths = _get_cache_paths(cache_base, search_paths)
    search_paths.extend(cache_paths)
    
    for path_name, path in search_paths:
        result = _search_for_pattern(path, pattern)

        if result:
            return result
    
    return None


def decrypt_game_config(data: bytes) -> str:
    if data is None:
        raise ValueError("Game config data is None. Make sure the APK is downloaded and extracted properly.")
        
    encryption_service = TableEncryptionService()
    encoded_data = b64encode(data)

    game_config = encryption_service.create_key('GameMainConfig')
    server_data = encryption_service.create_key('ServerInfoDataUrl')

    decrypted_data = encryption_service.convert_string(encoded_data, game_config)
    loaded_data = json.loads(decrypted_data)

    decrypted_key = encryption_service.new_encrypt_string('ServerInfoDataUrl', server_data)
    decrypted_value = loaded_data[decrypted_key]
    return encryption_service.convert_string(decrypted_value, server_data)


def catalog_url(version: str = None) -> str:
    return decrypt_game_config(find_game_config(version))