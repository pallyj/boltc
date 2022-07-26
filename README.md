# Bolt

<img src="https://github.com/pallyj/boltc/blob/main/assets/logo.svg?raw=true" width="256" height="256"/>

Bolt is a powerful coding language designed for expressiveness and speed. Bolt combines the ease-of-use of functional programming with the speed of C to make the ultimate language, suitable for any use-case. Bolt is inspired by Swift, Rust, Kotlin, and OCaml. The current version features:

- immutable-by-default variables
- first-class functions
- algebraic data structures
- pattern matching
- primitive types
- first-class tuples and arrays
- closures
- trailing closure syntax
- macros
- loops
- guard statement
- overloadable operators
- compiler extensions

The first releases of Bolt target:

- Fast prototyping
- Scripting
- Webpages

We are looking to expand into:

- UI Development
- Distributed computing
- Servers

Check out the [wiki](https://github.com/pallyj/boltc/wiki) for more details, or the documentation for our [runtime](https://pallyj.github.io/boltdoc/runtime/)!

The Bolt Language focuses on

- Safety
- Expressiveness
- Accessibility
- Speed

## Safety

Bolt is designed with safety in mind. All types are statically determined, and implicit conversion is frowned upon. Bolt has pointer safety, preventing dangerous null pointer exceptions and making nil checking mandatory. Bolt uses automatic reference counting to prevent memory leaks and ensure memory safety.
## Expressiveness

The Bolt language is a fresh, fun take on speedy coding languages. Code is easy to read and understand at a glance. The compiler can be tuned to squeeze the most possible performance out of your code 

## Accessibility

Bolt code is easily accessible for beginners and the experienced alike. Beginners will instinctively get the javascript-like syntax and python like methodology, while being able to code in a way that feels natural. Advanced coders will appreciate the built-in package management and C interoperability.

## Speed

The Bolt compiler is optimized for blazing fast speed. Bolt is written quickly, compiles quickly, and runs quickly. Bolt uses the LLVM framework to output the fastest code possible, and an interpreter for the fastest development. In some workloads, Bolt approaches the performance of C.

## Benchmarks

| Name          | Bolt 0.4.0 | Bolt 0.6 | C      | Rust   | Javascript |
|---------------|------------|----------|--------|--------|------------|
| factorial/sec | 7.9M	     | 2.5M     | 8.0M   | 1.6M   | 310k	   |
| speedup       | 1x		 |          | 0.98x  | 4.9x   | 25x		   |

Even in its early stages, Bolt is blazing fast. Bolt runs the factorials example 25 times faster than nodejs (!), 4.9 faster than rust, and with the margin of error from C.

Bolt will be as fast C in CPU-bound tasks, faster than Rust and Javascript in IO-bound tasks, and significantly faster than javascript in memory-bound tasks.

## Using

First, we need to download the Bolt source code. Open a command prompt and type

```
> cd ~/Documents
> git clone https://github.com/pallyj/boltc
> cd boltc
```

Now, the Bolt compiler needs to be installed to the system. Run the command

```
> cargo install .
```

and `boltc` will be available in the PATH. However, it needs to be run in the source code directory to install the standard library.

```
> boltc install
```

And your done! boltc can be run from any directory on your computer. To get started, run

```
boltc examples/guessing_game/game.bolt --lib=game
```

## Development

### Bolt 0.6 [ Current Version ]

- Loops
- Mutating functions
- Mutable variables
- RawPointers
- Virtual Machine
- Guard
- If and guard pattern matching
- Arrays
- Boltdoc
- Macros

- Completeness checks
- Global Variables
- Bolt playgroud

### Bolt 0.5

- Patterns
- Match
- Enums
- Strings
- Chars
- Tuples
- Index operator
- Input API
- Type Aliases
- Compiler plugins
- Documentation
- Web Playground

### Bolt 0.4

- Operators
- Polymorphism
- Function parameter labels
- Globals
- Initializers
- Closures