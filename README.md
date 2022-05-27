# Bolt

<img src="https://github.com/pallyj/boltc/blob/main/assets/logo.svg?raw=true" width="256" height="256"/>

Bolt is a powerful coding language designed for expressiveness and speed. Bolt combines the ease-of-use of functional programming with the speed of C to make the ultimate language, suitable for any use-case. Bolt is inspired by Swift, Rust, Kotlin, and OCaml. The current version features:

- immutable variables
- integer, float, and string types
- first-class functions
- algebraic data structures
- overridable operators
- pattern matching
- closures
- tuples
- compiler extensions

Bolt eventually wants to be usable in any domain, but the first areas we are targeting are:

- Scripting
- Fast webpages

Check out the [wiki](https://github.com/pallyj/boltc/wiki) for more details!

The Bolt Language focuses on

- Safety
- Expressiveness
- Accessibility
- Safety

## Safety

Bolt is designed with safety in mind. All types are statically determined, and implicit conversion is frowned upon. Bolt has pointer safety, preventing dangerous null pointer exceptions and making nil checking mandatory. Bolt uses automatic reference counting to prevent memory leaks and ensure memory safety.
## Expressiveness

The Bolt language is a fresh, fun take on speedy coding languages. Bolt code is easy to write and understand, and just looks better than any other language. The compiler can be extended to support low-level features, with attribute semantics to add them to your code.

## Accessibility

Bolt code is easily accessible for beginners and the experienced alike. Beginners will instinctively get the javascript-like syntax and python like methodology, while being able to code in a way that feels natural. Advanced coders will appreciate the built-in package management and C interoperability.

## Speed

The Bolt compiler is optimized for speed. Pure, raw speed. Quick building, quick execution, and quick development. Bolt uses the LLVM framework to output the fastest code possible. In CPU-bound workloads, Bolt approaches the speed of C, while in memory-bound tasks it beats C# and Java.

## Benchmarks

| Name          | Bolt 0.4.0 | C      | Rust   | Javascript |
|---------------|------------|--------|--------|------------|
| factorial/sec | 7.9M	     | 8.0M   | 1.6M   | 310k		|
| speedup       | 1x		 | 0.98x  | 4.9x   | 25x		|

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

### Bolt 0.5 [ Current Version ]

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