# scratchnative
<center> Compile scratch3 code to native executables.</center>

## Goal
The goal of this project is to safely compile Scratch3 projects to fast, native C++ code.

## Used in

- [ScratchOS](https://github.com/scratchnative/scratchOS), an actual operating system kernel written in Scratch

- WIP: Linux kernel modules in scratch

## Supported opcodes

- [x] `event_whenflagclicked`
- [x] `data_setvariableto`
- [x] `data_showvariable`
- [x] `operator_add`
- [x] `operator_substract`
- [x] `operator_multiply`
- [x] `operator_divide`
- [x] `operator_lt`
- [x] `operator_equals`
- [x] `operator_gt`
- [x] `operator_and`
- [x] `operator_or`
- [x] `operator_not`
- [x] `operator_random` (TODO)
- [x] `operator_join` (TODO)
- [x] `operator_letter_of` (TODO)
- [x] `operator_length` (TODO)
- [x] `operator_contains` (TODO)
- [x] `operator_mod` (TODO)
- [x] `operator_round` (TODO)
- [x] `control_if`
- [x] `control_repeat`
- [x] Extern C functions through `procedures_call`
- [ ] the rest

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
