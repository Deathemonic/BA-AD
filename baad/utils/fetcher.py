import json
from base64 import b64encode

from bacy import convert_string, new_encrypt_string, create_key
from .. import __app_name__, __app_author__
from ..helpers.filemanager import get_data_dir


def find_game_config() -> None | bytes:
    pattern = bytes([
        0x47,
        0x61,
        0x6D,
        0x65,
        0x4D,
        0x61,
        0x69,
        0x6E,
        0x43,
        0x6F,
        0x6E,
        0x66,
        0x69,
        0x67,
        0x00,
        0x00,
        0x92,
        0x03,
        0x00,
        0x00,
    ])
    cache_dir = get_data_dir(__app_name__, __app_author__)
    game_path = cache_dir / 'data' / 'assets' / 'bin' / 'Data'

    for config_file in game_path.rglob('*'):
        if config_file.is_file():
            content = config_file.read_bytes()

            if pattern in content:
                start_index = content.index(pattern)
                data = content[start_index + len(pattern):]
                return data[:-2]
    return None


def decrypt_game_config(data: bytes) -> str:
    encoded_data = b64encode(data).decode()

    game_config = create_key(b'GameMainConfig')
    server_data = create_key(b'ServerInfoDataUrl')

    decrypted_data = convert_string(encoded_data, game_config)
    loaded_data = json.loads(decrypted_data)

    decrypted_key = new_encrypt_string('ServerInfoDataUrl', server_data)
    decrypted_value = loaded_data[decrypted_key]
    return convert_string(decrypted_value, server_data)


def catalog_url() -> str:
    return decrypt_game_config(find_game_config())