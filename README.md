# mkgif
[![Rust application](https://github.com/jkawamoto/mkgif/actions/workflows/ci.yaml/badge.svg)](https://github.com/jkawamoto/mkgif/actions/workflows/ci.yaml)

Create an animation GIF from the given image files.

## Usage

```
Usage: mkgif [OPTIONS] [PATHS]...

Arguments:
[PATHS]...  Paths to the input image files

Options:
-o, --output <FILE>  Path to the output file [default: output.gif]
-s, --speed <SPEED>  Processing speed in [1, 30]. The higher the value the faster it runs at the cost of image quality [default: 10]
-h, --help           Print help
-V, --version        Print version
```

## Installation
```shell
cargo install --git https://github.com/jkawamoto/mkgif
```

## License
This application is released under the MIT License. For details, see the [LICENSE](LICENSE) file.
