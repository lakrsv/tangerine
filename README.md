# Tangerine

## Developing
Make sure rust is installed (rustup recommended)
```https://www.rust-lang.org/tools/install```
Install SDL2 and SDL2 Image
```https://wiki.libsdl.org/SDL2/Installation```

The binaries for SDL2 (SDL2.lib, SDL2.dll, SDL2_image.lib, SDL2_image.dll) have to be placed in the rustup toolchain. On Windows, this is usually located
here: ```C:\Users\lakrs\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib```

To verify everything is working as intended, run
```
cargo build
cargo test
```

## Building
In the root of the repository, simply run
```cargo build```
## Testing
In the root of the repository, simply run
```cargo test```