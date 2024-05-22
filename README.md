# mkgif
[![Build](https://github.com/jkawamoto/mkgif/actions/workflows/build.yaml/badge.svg)](https://github.com/jkawamoto/mkgif/actions/workflows/build.yaml)

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

```bash
brew install jkawamoto/tap/mkgif
```

Prebuilt binaries are also available on the [release page](https://github.com/jkawamoto/mkgif/releases).

## License
This application is released under the MIT License. For details, see the [LICENSE](LICENSE) file.
