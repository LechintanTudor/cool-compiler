# 🧊 Cool Compiler

Reference compiler for the Cool Programming Language.

Disclaimer: This project is in development. There are bugs and missing features
that will be addressed in future releases.

# 🧊 Cool Language

Cool is a systems programming languge aiming to provide a modern development
environment for system software while keeping a small core to ensure
portability.

```
printf :: extern fn(format: *i8, ...) -> i32;

export main :: fn() {
    printf(c"Hello, world!\n");
};
```

## Language Design & Features

### Readable syntax.

```
printf :: extern fn(format: *i8, ...) -> i32;

Point :: struct {
    x: f32,
    y: f32,
};

is_origin :: fn(point: Point) -> bool {
    point.x == 0 && point.y == 0
};

export main :: fn() {
    point := Point { x = 0, y = 0 };

    if is_origin(point) {
        printf(c"Point is the origin\n");
    } else {
        printf(c"Point is not the origin\n");
    }
};
```

### File-based module system.

- File: `main.cl`

```
libc :: module;

export main :: fn() {
    libc.printf(c"Hello, world!\n");
};
```

- File: `libc.cl` or `libc/@module.cl`

```
printf :: extern fn(format: *i8, ...) -> i32;
```

### Simple type inferrence for functions and variables.

```
sum1 :: fn(a: i32, b: i32) -> i32 {
    a + b
};

sum2: fn(i32, i32) -> i32: fn(a, b) {
    a + b
};

sum3: fn(i32, i32) -> i32: fn(a: i32, b: i32) -> i32 {
    a + b
};
```

### Blocks are expressions.

```
get_sign_str :: fn(n: i32) -> *i8 {
    if n < 0 {
        c"negative"
    } else if n == 0 {
        c"zero"
    } else {
        c"positive"
    }
};
```

# 🧊 Compiler Implementation

The reference Cool Compiler is written in Rust and targets LLVM. The compiler
reads UTF-8 encoded source files and transforms them into parse trees using a
hand-written recursive descent algorithm. After module and import resolution and
some initial type-checking, the parse trees are converted into an abstract
syntax tree that represents a valid Cool program. This conversion process
includes name resolution and type-checking for local variables, as well as
mutability checks for constant objects and pointers to constant objects.
Finally, the compiler generates LLVM IR by going over the prviously created
abstract syntax tree and using the LLVM builder API provided by the
[Inkwell](https://crates.io/crates/inkwell) crate.

## Various Implementation Details

- The lexer requires 2 characters of lookahead to generate a token.
- The parser requires 1 token of looahead to parse the token stream.

# 🧊 License

This project is licensed under the [Mozilla Public License 2.0](LICENSE).
