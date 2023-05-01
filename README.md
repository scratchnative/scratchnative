# Scratchnative

_now in rust!_

Scratchnative is an ongoing project to compile Scratch3 programs to native executables.

## How does it work

Here's how Scratchnative is able to compile Scratch3 projects:

1. It serializes the `project.json` file contained in the `.sb3` Scratch file (which is a zip) using serde
2. It builds an abstract syntax tree (AST)
3. It outputs C++ code by iterating through the AST

Eventually, I will write documentation on the `project.json` file format as the current docs are quite vague and unclear.

## State of the project

Right now, Scratchnative is able to compile very basic Scratch3 projects (e.g basic operators, loops and expressions), albeit without graphics, to C++ with the help of a runtime.
