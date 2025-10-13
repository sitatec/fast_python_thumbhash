from pathlib import Path

try:
    from PIL import Image, ImageOps

    _HAS_PIL = True
except ImportError:
    _HAS_PIL = False

# Import the Rust extension module directly
import fast_thumbhash as _rust_mod


def image_to_thumb_hash(fp: str | bytes | Path) -> bytes:
    """
    Opens given image file and encodes to a ThumbHash
    """
    if not _HAS_PIL:
        raise ImportError("Pillow not installed, please re-install with pillow extra")

    img = Image.open(fp)

    img = img.convert("RGBA")
    img.thumbnail((100, 100))

    img = ImageOps.exif_transpose(img)

    thumb_hash = rgba_to_thumb_hash(img.width, img.height, img.tobytes())

    return thumb_hash


def rgba_to_thumb_hash(width: int, height: int, data: list[int] | bytes | bytearray) -> bytes:
    if isinstance(data, list):
        data = bytes(data)
    return _rust_mod.py_rgba_to_thumb_hash(width, height, data)


def thumb_hash_to_rgba(thumb_hash: bytes) -> tuple[int, int, bytes]:
    return _rust_mod.py_thumb_hash_to_rgba(thumb_hash)


def thumb_hash_to_average_rgba(thumb_hash: bytes) -> tuple[int, int, int, int]:
    return _rust_mod.py_thumb_hash_to_average_rgba(thumb_hash)


def thumb_hash_to_approximate_aspect_ratio(thumb_hash: bytes) -> float:
    return _rust_mod.py_thumb_hash_to_approximate_aspect_ratio(thumb_hash)
