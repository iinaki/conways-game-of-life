# Conway's Game of Life, in Rust

### The rules of the game
- Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

### How to run
- Clone the repository with `git clone`
- Enter the directory containing the game files and run `cargo run`

### Controls
- 'SPACE' to pause/unpause the game.
- 'E' to enter/exit Edit Mode, here you can place new cells and remove existing ones by clicking on them.
- 'X' to exit the game.
- 'R' to restart the game with a new randomly generated seed.
