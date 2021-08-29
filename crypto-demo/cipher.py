import base64
from cryptography.hazmat import backends
from cryptography.hazmat.primitives import ciphers
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend

key = base64.decodebytes(b'bDkbHkA9KoHXH8ZSz61wQQ==')
nonce = base64.decodebytes(b'miFPm8SPZNc3w4sHOOrt8w==')
ct = base64.decodebytes(b'spRaxA==')
backends = default_backend()
ciphers = Cipher(algorithms.AES(key), modes.CTR(nonce), backend=backends)
decryptor = ciphers.decryptor()
print(decryptor.update(ct) + decryptor.finalize(), b'abcd')