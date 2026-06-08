# Chess

A chess game written in Rust, using [Bevy](https://bevyengine.org/) for windowing and rendering.

Chess is a two-player strategy board game played on an 8x8 grid. Each side starts with
16 pieces — a king, a queen, two rooks, two bishops, two knights, and eight pawns — and
the goal is to checkmate the opponent's king: trap it so it's under attack with no way
to escape.

## Features

A windowed chess game supporting two modes: player vs. player (local, same machine) and
player vs. a computer opponent.

## Running

```
cargo run
```

## Tech

- Rust
- [Bevy](https://bevyengine.org/) (ECS game engine — handles the window, rendering, and input)
