# imgcnvrt

`imgcnvrt` is a tiny CLI for doing fast, efficient conversions between image file formats. It is written in [Rust](https://www.rust-lang.org/) and extends functionality from the lovely [`image`](https://github.com/image-rs/image) library.

## Installation

To install `imgcnvrt`, first ensure you have a working installation of Rust and [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html), Rust's build system and package manager. Follow [the instructions from the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).

Next, run:

```sh
cargo install imgcnvrt
```

## Running `imgcnvrt`

Once you have `imgcnvrt` installed, you can run it like so:

```sh
imgcnvrt path/to/my/image.jpg .png
```

`imgcnvrt` takes in two arguments:

1. The path to the source image.
2. The image type you want to convert to (e.g. `.png`, `.jpeg`, `.webp`, `.avif`)

### Supported File Formats

`imgcnvrt` supports converting from and to all of the file formats handled by its parent library [`image`](https://github.com/image-rs/image#supported-image-formats). This includes:

- PNG
- JPEG
- GIF
- BMP
- ICO
- TIFF
- WebP
- AVIF
- PNM
- DDS
- TGA
- OpenEXR
- farbfeld
