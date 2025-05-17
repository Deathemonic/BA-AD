# from _typeshed import StrPath
from io import BytesIO
from typing import IO
from zipfile import ZipFile
import os
from .XXHashService import calculate_hash
from typing import Union
from .MersenneTwister import MersenneTwister
from base64 import b64encode

class TableZipFile(ZipFile):
    def __init__(self, file: Union[str, BytesIO], name: str = None) -> None:
        super().__init__(file)
        file_hash = calculate_hash(os.path.basename(file))                   # Calculate file hash with zipfile name
        rand = MersenneTwister(file_hash)                                    # Initialize a MT generator
        bts = rand.NextBytes(15)                                             # Generate a 15-byte key
        self.password = b64encode(bts)                                       # Encode in BASE64 to get true password
        print(f'Password cracked: {self.password} for file ->{os.path.basename(file)}<-')

    def open(self, name: str, mode: str = "r", force_zip64=False):
        return super(self.__class__, self).open(
            name, mode, pwd=self.password, force_zip64=force_zip64
        )
