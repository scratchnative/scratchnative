# scratchnative
<center> Compile scratch3 code to native executables.</center>

## Goal
The goal of this project is to safely compile Scratch3 projects to fast, native C++ code.

## Used in

- [ScratchOS](https://github.com/scratchnative/scratchOS), an actual operating system kernel written in Scratch

- WIP: Linux kernel modules in scratch

## Supported opcodes

- `event_whenflagclicked` [X]
- `data_setvariableto` [X]
- `data_showvariable` [X]
- `operator_add` [X]
- `operator_substract` [X]
- `operator_multiply` [X]
- `operator_divide` [X]
- `operator_lt` [X]
- `operator_equals` [X]
- `operator_gt` [X]
- `operator_and` [X]
- `operator_or` [X]
- `operator_not` [X]
- `operator_random` [X] (TODO)
- `operator_join` [X] (TODO)
- `operator_letter_of` [X] (TODO)
- `operator_length` [X] (TODO)
- `operator_contains` [X] (TODO)
- `operator_mod` [X] (TODO)
- `operator_round` [X] (TODO)
- `control_if` [X]
- `control_repeat` [X]
- TODO: the rest [ ]

## How to use
First, you need to build the program using cmake (Look it up if you don't know how).

### Program usage:
```
scratchnative [OPTIONS] [INPUT]

OPTIONS:
    --freestanding: enable freestanding mode (compile without libc)
    -o: set output file

INPUT: input file, defaults to project.json
```

## Limitations
- Currently, scratchnative doesn't support any kind of graphics.
- Compile-time errors could be largely improved.
