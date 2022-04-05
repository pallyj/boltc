# Bolt

<img src="https://github.com/pallyj/boltcc/blob/main/assets/logo.svg?raw=true" width="256" height="256"/>

Bolt is a powerful coding language designed for expressiveness and speed. Bolt combines the ease-of-use of functional programming with the speed of C to make the ultimate language, suitable for any use-case. Bolt takes influence from Swift, Rust, and OCaml. Bolt is in the early stages of development, and the current version contains

- integer and float math
- immutable variables
- first-class functions

The Bolt Language focuses on

- Safety
- Expressiveness
- Accessibility
- Safety


## Safety

Bolt is designed with safety in mind. All types are determined at compile time, and implicit conversion is frowned upon. Bolt has no concept of null pointers, the single most dangerous aspect of programming. Bolt uses automatic reference counting to manage memory safely, preventing leaks and dangling pointers.
## Expressiveness

Bolt syntax is designed to concisely show what the program does, without excess verbosity. Bolt guesses the developers intent, and annotations can be added to specify it when the compiler is wrong.

## Accessibility

Bolt code is easy to understand for both beginners and those coming over from another language. The compiler has built-in package management, and can compile C code.

## Speed

The Bolt language is compiled ahead of time, using a LLVM backend to output optimized code for any backend. The lack of an inefficient garbage collector prevents lengthy pauses and cuts down on memory usage.

## Using

First, compile the standard library. Run std/compile.sh and libprint.o will be added to /bin.

With the standard library compiled, a test program can be run with

```
cargo run -- test/main.bolt --lib=test
```

## Development

### Bolt 0.5

- Patterns
- Match
- Enums

### Bolt 0.4

- Operators
- Polymorphism
- Globals
- Initializers

### Bolt 0.3.2

- Attributes
- Use static methods as values
- Handle voids and ifs in returns

### Bolt 0.3.1 [Current Version]

- Error Catching
- Visibilities
- Comments
- First-Class functions

### Bolt 0.3

- Rewrite compiler
- Type checking
- New lower level IR
- Codeblock analysis

### Bolt 0.2

- Floating point types
- Boolean functions
- Name mangling
- Structs
- Methods
- External functions
- Default literal initializers

### Bolt 0.1

- Integer types
- Type interference
- Function calls
- Compiles to LLVM