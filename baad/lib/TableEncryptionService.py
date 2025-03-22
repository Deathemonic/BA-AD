from base64 import b64decode, b64encode
from struct import Struct
from typing import Union

from Crypto.Util.strxor import strxor

from .MersenneTwister import MersenneTwister
from .XXHashService import calculate_hash


class TableEncryptionService:
    def __init__(self) -> None:
        self._struct_formats = {
            'uint32': Struct('<I'),
            'uint64': Struct('<Q'),
            'int32': Struct('<i'),
            'int64': Struct('<q'),
            'float32': Struct('<f'),
            'float64': Struct('<d'),
            'int8': Struct('<b'),
            'uint8': Struct('<B'),
        }

    def create_key(self, name: str) -> bytes:
        seed = calculate_hash(name)
        return MersenneTwister(seed).next_bytes(8)

    def xor(self, name: str, data: bytes) -> bytes:
        seed = calculate_hash(name)
        key = MersenneTwister(seed).next_bytes(len(data))
        return self._xor(data, key) if data else data

    def _xor(self, value: bytes, key: bytes) -> bytes:
        if len(value) == len(key):
            return strxor(value, key)

        elif len(value) < len(key):
            return strxor(value, key[: len(value)])

        return b''.join(
            strxor(value[i : i + len(key)], key) for i in range(0, len(value) - len(key) + 1, len(key))
        ) + strxor(
            value[(len(value) - (len(value) % len(key))) :],
            key[: len(value) % len(key)],
        )

    def _calculate_modulus(self, key: bytes) -> int:
        if key == b'':
            return 1
            
        modulus = key[0] % 10
        if modulus <= 1:
            modulus = 7
            
        if key[0] & 1:
            modulus = -modulus
            
        return modulus

    def _xor_struct(self, value: Union[int, float], key: bytes, struct_format: str) -> Union[int, float]:
        struct = self._struct_formats[struct_format]
        return struct.unpack(self._xor(struct.pack(value), key))[0]

    def convert_int(self, value: int, key: bytes) -> int:
        return self._xor_struct(value, key, 'int32') if value else 0

    def convert_long(self, value: int, key: bytes) -> int:
        return self._xor_struct(value, key, 'int64') if value else 0

    def convert_uint(self, value: int, key: bytes) -> int:
        return self._xor_struct(value, key, 'uint32') if value else 0

    def convert_ulong(self, value: int, key: bytes) -> int:
        return self._xor_struct(value, key, 'uint64') if value else 0

    def convert_ubyte(self, value: int, key: bytes) -> int:
        return self._xor_struct(value, key, 'uint8') if value else 0
        
    def convert_float(self, value: float, key: bytes) -> float:
        modulus = self._calculate_modulus(key)
        return (value / modulus) / 10000.0 if value and modulus != 1 else value

    def convert_double(self, value: float, key: bytes) -> float:
        return self.convert_float(value, key)

    def encrypt_float(self, value: float, key: bytes) -> float:
        modulus = self._calculate_modulus(key)
        return (value * 10000.0) * modulus if value and modulus != 1 else value

    def encrypt_double(self, value: float, key: bytes) -> float:
        return self.encrypt_float(value, key)

    def convert_string(self, value: str | bytes, key: bytes) -> str:
        if not value:
            return ''

        try:
            raw = b64decode(value)
            return self._xor(raw, key).decode('utf-16')

        except Exception:
            return value.decode('utf-8')

    def encrypt_string(self, value: str, key: bytes) -> str:
        return self.decrypt_string(value, 'utf-16', key)

    def new_encrypt_string(self, value: str, key: bytes) -> str:
        return self.decrypt_string(value, 'utf-16-le', key)

    def decrypt_string(self, value: str, encoding: str, key: bytes) -> str | bytes:
        if not value or len(value) < 8:
            return value.encode() if value else b''

        raw = value.encode(encoding)
        return b64encode(self._xor(raw, key)).decode()
