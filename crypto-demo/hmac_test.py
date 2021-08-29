import base64
import hashlib
import hmac

key = base64.decodebytes(b'w4uSJO7mqxEDqSmnqfCXtwYqfqk7qd7V3tknP+G1ktw=')
message = b'abcd'
hmac_digest = 'a8396110435429be32756bc09159acfb5216c732fbd4a4882808c2a0a9daecfa'

h = hmac.new(key, message, hashlib.sha256)

print(hmac.compare_digest(h.hexdigest(), hmac_digest))