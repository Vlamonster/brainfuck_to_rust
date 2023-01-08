# How to Run

```shell
# Compile and run the brainfuck from path. Results will be put in output folder.
cargo run --release -- <path> -cr
cargo run --release -- examples/sierpinski.bf -cr

# Just run (if output already exists) or compile.
cargo run --release -- <path> -r
cargo run --release -- <path> -c
```

# Included Examples

* add.bf
    * Example from Wikipedia that adds two cells together.
* hello_world.bf
    * Example from Wikipedia that prints out "Hello World!".
* sierpinski.bf
    * Example found [here](https://www.msx.org/wiki/BrainFuck) that draws a Sierpinski triangle with code in the shape
      of a Sierpinski triangle.
* tictactoe.bf
  * Example from Daniel B. Cristofani (2020) that lets the user play a game of tic-tac-toe.
* mandelbrot.bf
  * Example from Erik Bosman that draws the Mandelbrot set. I suggest compiling and running separately.