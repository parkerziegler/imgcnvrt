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
2. The image type you want to convert to (e.g. `.png`, `.jpeg`, `.tiff`)

### Supported File Formats

`imgcnvrt` supports convertions based on the restrictions of its parent library [`image`](https://github.com/image-rs/image#supported-image-formats). Depending on the format you are decoding (converting from) and encoding (converting to), you may encounter errors. For example, there is no support for encoding WebP formats in `image` yet; therefore, `imgcnvrt` does not support it either. Likewise, if you want to decode a WebP image, it must not contain animation or alpha headers.

For a full list of decoding and encoding support, check out `image`'s [supported image formats](https://github.com/image-rs/image#supported-image-formats).