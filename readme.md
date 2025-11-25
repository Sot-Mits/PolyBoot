# PolyBoot

**PolyBoot is a free and open source tool for Linux x86_64 systems that aims to be easy and simple to build, read create bootable USB devices, without relying on external tooling.**

As such, support may be quite limited, but given the nature of such a low-level tool it is paramount that everyone can easily understand the program and not fall in the, "it's open-source therefore it's secure", fallacy.

## Compilation

Building PolyBoot is simple to build by design and doesn't obfuscate the build process; Everything needed to build and run PolyBoot is included or pulled automatically so you don't have to deal with dependencies.

``` bash
git clone https://github.com/Sot-Mits/PolyBoot.git
cd PolyBoot
cargo build --release
```

## Usage

``` bash
polyboot [BLOCK_DEVICE] [IMAGE_DIRECTORY]
```

### Examples

Initialising ```/dev/sda```:

``` bash
sudo ./polyboot /dev/sda
```

Initialising ```/dev/sda``` and copying images from ```~/Documents/ISOs```:

``` bash
sudo /polyboot /dev/sda ~/Documents/ISOs
```
