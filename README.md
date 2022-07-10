An experimental raytracer written in Rust

## Installation instructions (Windows)

- Install **Rust**
- Install **MinGW**
- Follow **Windows (MinGW)** instructions from: https://crates.io/crates/sdl2
- Run
    > rustup uninstall toolchain stable-x86_64-pc-windows-msvc <br>
    > rustup toolchain install stable-x86_64-pc-windows-gnu <br>
    > rustup default stable-x86_64-pc-windows-gnu

## Tools used

- Graphics : **SDL2**
- Inputs : **SDL2**
- Parallelism (CPU) : **Rayon**
