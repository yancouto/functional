# functional
A game about lambda calculus.

Status: Alpha

## How to build

## Steam integration

Use the "steam" feature if you want to test Steam integration. If using it, you must have the SDK downloaded and the `STEAM_SDK_LOCATION` env var pointing to it (on Windows, make it an absolute path) (if using `-Z configurable-env`, it defaults to the `sdk` directory in the root).

You must also have the libraries (like `steam_api.dll`) on the create root.

### Windows

If using steam, you need clang installed. The easiest way to install it is through chocolatey:
- Install chocolatey from [here](https://chocolatey.org/install#individual)
- Run `choco install llvm`

### Linux

For the clipboard integration, you need some x11 dependencies. You might already have some or all of them, if not, on Ubuntu-like systems you can install them with:
- `sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev`

## How to run
```
cargo run
```

