# Rust Password Generator

A secure password generator written in Rust that allows users to create random passwords with customizable options.

## Features

- Configurable password length (8-128 characters)
- Options to include/exclude:
  - Uppercase letters
  - Numbers
  - Special characters
- Always includes lowercase letters for readability
- Uses Rust's secure random number generator

## Requirements

- Rust and Cargo installed on your system

## Installation

1. Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/rustyLock.git
cd rustyLock
```

2. Build the project:

```bash
cargo build --release
```

## Usage

Run the program:

```bash
cargo run
```

The program will prompt you for:

- Password length (defaults to 12 if no input is provided)
- Whether to include uppercase letters (default: yes)
- Whether to include numbers (default: yes)
- Whether to include special characters (default: yes)

## Roadmap

- Make it General to create random string for security application like (https://djecrety.ir/)
- Support popular framework requirement of secreat string like Django SECRET_KEY (minimum 50 suggested)
- Support other cryptographic requirement like :
  - token_urlsafe() method from python secrets module
  - get_random_secret_key() from django.core.management.util

```python
# secrets module

def token_urlsafe(nbytes=None):
    """Return a random URL-safe text string, in Base64 encoding.

    The string has *nbytes* random bytes.  If *nbytes* is ``None``
    or not supplied, a reasonable default is used.

    >>> token_urlsafe(16)  #doctest:+SKIP
    'Drmhze6EPcv0fN_81Bj-nA'

    """
    tok = token_bytes(nbytes)
    return base64.urlsafe_b64encode(tok).rstrip(b'=').decode('ascii')

def urlsafe_b64encode(s):
    """Encode bytes using the URL- and filesystem-safe Base64 alphabet.

    Argument s is a bytes-like object to encode.  The result is returned as a
    bytes object.  The alphabet uses '-' instead of '+' and '_' instead of
    '/'.
    """
    return b64encode(s).translate(_urlsafe_encode_translation)

def urlsafe_b64decode(s):
    """Decode bytes using the URL- and filesystem-safe Base64 alphabet.

    Argument s is a bytes-like object or ASCII string to decode.  The result
    is returned as a bytes object.  A binascii.Error is raised if the input
    is incorrectly padded.  Characters that are not in the URL-safe base-64
    alphabet, and are not a plus '+' or slash '/', are discarded prior to the
    padding check.

    The alphabet uses '-' instead of '+' and '_' instead of '/'.
    """
    s = _bytes_from_decode_data(s)
    s = s.translate(_urlsafe_decode_translation)
    return b64decode(s)

    # Django

def get_random_secret_key():
    """
    Return a 50 character random string usable as a SECRET_KEY setting value.
    """
    chars = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*(-_=+)"
    return get_random_string(50, chars)


RANDOM_STRING_CHARS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
def get_random_string(length, allowed_chars=RANDOM_STRING_CHARS):
    """
    Return a securely generated random string.

    The bit length of the returned value can be calculated with the formula:
        log_2(len(allowed_chars)^length)

    For example, with default `allowed_chars` (26+26+10), this gives:
      * length: 12, bit length =~ 71 bits
      * length: 22, bit length =~ 131 bits
    """
    return "".join(secrets.choice(allowed_chars) for i in range(length))

```

## License

MIT License
