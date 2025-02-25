from base64 import b64encode
from io import BytesIO
from pathlib import Path
from typing import Union
from zipfile import ZipFile

from .MersenneTwister import MersenneTwister
from .XXHashService import calculate_hash


class TableZipFile(ZipFile):
    def __init__(self, file: Union[str, BytesIO], password: bytes = None) -> None:
        super().__init__(file)
        if password is not None:
            self.password = password
            return
            
        file_name = Path(file).name if isinstance(file, str) else file.name
        self.password = self._generate_password(file_name.lower())

    def _generate_password(self, file_name: str) -> bytes:
        hash_value = calculate_hash(file_name)
        twister = MersenneTwister(hash_value)
        next_bytes = twister.next_bytes(15)
        return b64encode(next_bytes)

    def open(self, name: str, mode: str = 'r', force_zip64: bool = False) -> bytes:
        return super().open(name, mode, pwd=self.password, force_zip64=force_zip64)
