# Conway's Game of Life
A rust implementation of Conway's game of life using macroquad
## Build Instructions
Clone the Repo and then run:
```
cargo build
cargo run
```
## Usage Instructions
Click or drag with your mouse to turn on or "populate" cell's and press space to start or pause the simulation. Right clicking on a cell turns it off.

## Game of life algorithm
The game of life takes place in an infinite two dimensional grid of cells. However in this implementation the grid is finitely large. every cell relates to 8 of it's direct neighbors
<img src="grid.png" alt="drawing" width="200" text-align="center"/>
