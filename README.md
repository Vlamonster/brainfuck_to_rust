# How to Run
```shell
# Compile and run the brainfuck from path. Results will be put in output folder.
cargo run --release -- <path> -cr
cargo run --release -- examples/hello_world.bf -cr

# Just run (if output already exists) or compile.
cargo run --release -- <path> -r
cargo run --release -- <path> -c
```
Currently only support for Windows. I will see if I can get it too work on Linux too soon.