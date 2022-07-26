# Bolt

<img src="https://github.com/pallyj/boltc/blob/main/assets/logo.svg?raw=true" width="256" height="256"/>

Bolt is a new progamming language designed for speed and beautiful code. Bolt combines the constructs of functional languages with the syntax and speed of C-like languages. Writing code in Bolt is easy for both newcomers and advanced programmers. Bolt takes inspiration from Swift, Rust, Kotlin, and OCaml. Features of the current (early alpha) version are:

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

Bolt will be usable for any purpose, but in its early version, it specifically targets fast prototyping, scripting, and writing webpages with wasm. After the 1.0 release, Bolt's development will focus on gui applications, distributed computing, and high-reliability servers.

Check out the [wiki](https://github.com/pallyj/boltc/wiki) for more details, or the documentation for our [runtime](https://pallyj.github.io/boltdoc/runtime/)!

The Bolt Language was designed with 4 goals in mind:

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

| Name          | Bolt 0.6 | C      | Rust   | Javascript |
|---------------|----------|--------|--------|------------|
| factorial/sec | 2.5M     | 8.0M   | 1.6M   | 310k	  	  |
| speedup       | 1x       | 0.31x  | 4.9x   | 25x		  |
| mandlebrot	| 52.06	   |        | 3.78   |			  |

Bolt's performance is not great in its early stages. With optimizations, it should experience huge speedups.

## Building

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
- Global Variables

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