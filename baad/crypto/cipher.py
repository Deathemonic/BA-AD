from base64 import b64decode, b64encode

from Crypto.Cipher import AES
from Crypto.Protocol.KDF import PBKDF2
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad

# TODO: Deprecated (Moving to BA-CY)
# Keep here for compatibility
def encrypt(plain_text: str, pass_phrase: str) -> str:
    block_size = 16

    salt = get_random_bytes(block_size)
    iv = get_random_bytes(block_size)

    derived = PBKDF2(pass_phrase, salt, 16, count=1000)
    cipher = AES.new(key=derived[:16], iv=iv, mode=AES.MODE_CBC)
    data = pad(cipher.encrypt(plain_text.encode('utf-8')), block_size, style='pkcs7')

    return b64encode(salt + iv + data).decode('utf-8')


def decrypt(cipher_text: str | bytes, pass_phrase: str) -> str:
    block_size = 16

    raw_cipher_text = b64decode(cipher_text)
    salt, iv, encrypted_data = (
        raw_cipher_text[:block_size],
        raw_cipher_text[block_size : block_size * 2],
        raw_cipher_text[block_size * 2 :],
    )

    derived = PBKDF2(pass_phrase, salt, 16, count=1000)
    cipher = AES.new(key=derived, iv=iv, mode=AES.MODE_CBC)

    return unpad(cipher.decrypt(encrypted_data), block_size, style='pkcs7').decode('utf-8')


def encrypt_string_to_bytes(plain_text: str, key: bytes, iv: bytes) -> bytes:
    block_size = 16

    cipher = AES.new(key=key, iv=iv, mode=AES.MODE_CBC)
    return pad(cipher.encrypt(plain_text.encode('utf-8')), block_size)


def decrypt_string_from_bytes(cipher_text: bytes, key: bytes, iv: bytes) -> str:
    block_size = 16

    cipher = AES.new(key=key, iv=iv, mode=AES.MODE_CBC)
    return unpad(cipher.decrypt(cipher_text), block_size).decode('utf-8')
