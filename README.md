# functional
A game about lambda calculus.

Status: Released

https://store.steampowered.com/app/1636730/functional/

## How to build

We use nightly Rust.

## Steam integration

Use the "steam" feature if you want to test Steam integration. If using it, you must have the SDK downloaded and the `STEAM_SDK_LOCATION` env var pointing to it (on Windows, make it an absolute path) (it defaults to the `sdk` directory in the root).

You must also have the libraries (like `steam_api.dll`) on the create root.

### Windows

If using steam, you need clang installed. The easiest way to install it is through chocolatey:
- Install chocolatey from [here](https://chocolatey.org/install#individual)
- Run `choco install llvm`

Install OpenAL as in [ears README](https://github.com/nickbrowne/ears#before-you-start), but use `nightly-gnu` rust toolchain version instead. You might need to copy several `.dll`s to this directory, check the errors when running and look in the `msys64` directory.

To publish, I use [this tool](https://learn.microsoft.com/en-gb/sysinternals/downloads/process-explorer) to find out which DLLs it loads, and everything that's from MSYS folder I copy next to the binary.

### Linux

For the clipboard integration, you need some x11 dependencies. You might already have some or all of them, if not, on Ubuntu-like systems you can install them with:
- `sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev`
For audio, you need openal and libsndfile. On Ubuntu-like, try:
- `sudo apt-get install libopenal-dev libsndfile1-dev`

### Mac

For ears, install:
- `brew install openal-soft libsndfile`

And set `PKG_CONFIG_PATH` as specified in the install hints.

To publish package, you need to copy the libsndfile and libopenal dylibs, and read [this tutorial](https://medium.com/@donblas/fun-with-rpath-otool-and-install-name-tool-e3e41ae86172) to use `install_name_tool` to use `@executable_path` for the dylibs provided together with the binary.

## How to run
```
cargo run
```

## How to test in codespace
You can minimally test this code in a Codespace, using:
```
cargo test --no-default-features --features crossterm
```
This will disable all the unecessary stuff (for tests) that need fancy setting up.