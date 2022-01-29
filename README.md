# boltcc

![Bolt logo](https://github.com/pallyj/boltcc/blob/main/assets/logo.svg?raw=true)

Boltcc is a C compiler updated for the modern day.

The most painful parts of writing C are keeping separate headers from the source code, and project management. Boltcc solves both of these issues.

## Project manager

Boltcc includes a project manager that can link with CMake and Bazel. What's more, boltcc can pull in dependencies from git and compile them for you. And, bolt can generate bindings for traditional c, rust, or other languages coming soon.

## Header generation

Boltcc gets around c headers by having one header per source file. This header is automatically generated from its source file. The single header will include any outside headers, as well as other files in the project.

## Speed

Boltcc is much faster than a regular C compiler. Boltcc cuts down on compiler time by not having to load lots of headers for each file. Bolt is written in rust, a fast, safe language, and designed for multithreading.

## Clang backend

The boltcc compiler is very new, (and not current working), so a clang backend is provided too. The clang backend lets you use the great features such as the project manager and header generation with a mature compiler.



## Features

- [x] C lexer

## WIP

- [ ] C preprocessor
- [ ] C parser

## TODO

- [ ] Error messages
- [ ] C LLVM Backend
- [ ] Clang backend
- [ ] Project system
- [ ] Headergen
- [ ] Sysroots and target files
- [ ] Multithreading