# Bolt

<img src="https://github.com/pallyj/boltcc/blob/main/assets/logo.svg?raw=true" width="256" height="256"/>

Bolt is a powerful coding language designed for expressiveness and speed. Bolt combines the ease-of-use of functional programming with the speed of C to make the ultimate language, suitable for any use-case.

## Using

First, compile the standard library. Run std/compile.sh and libprint.o will be added to /bin.

With the standard library compiled, a test program can be run with

```
cargo run -- test/main.bolt --lib=test
```

## Development

### Bolt 1.0

- Fix bugs
- Release-optimized code
- Finalized version 1.0 of lang and intrinsics

### Bolt 0.14

- Iterators
- For loop

### Bolt 0.13

- Dataflow operators
- Function currying
- Declarative function syntax

### Bolt 0.12

- C interop
- Better frontend

### Bolt 0.11

- Syntactic sugar for options/results
- Catch/Try
- Strings

### Bolt 0.10

- Lang library
- Project manager

### Bolt 0.9

- Classes

### Bolt 0.8

- Generics

### Bolt 0.7

- Protocols

### Bolt 0.6

- Mutable variables
- Loops
- Guard

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