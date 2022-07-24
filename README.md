# Predator Prey cellular automaton in Rust

I did this small hobby project to have some practice with Rust.

## Sources
This whole work is based on the README of [https://github.com/Hopson97/CellularAutomaton](https://github.com/Hopson97/CellularAutomaton).

## Running the code
The code can be run using the following command in the root of the repo:
```shell
$ cargo run --release
```

Some parameters, such as the size of the image, can be updated in the code.

## Converting to GIF
The image can be converted to gif using FFMPEG:
```shell
$ ffmpeg -i img/%08d.png output.gif
```

![](https://github.com/Sangelow/predator_prey/blob/master/img/output.gif)


