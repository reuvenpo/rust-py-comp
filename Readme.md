# py-comp - A Rust macro implementing a Python-like generator expression

[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

[crates.io]: https://crates.io/crates/py-comp
[Latest Version]: https://img.shields.io/crates/v/py-comp.svg
[Documentation]: https://docs.rs/py-comp/badge.svg
[docs.rs]: https://docs.rs/py-comp
[License]: https://img.shields.io/crates/l/py-comp.svg

This macro implements a syntax that emulates Python's
[`generator-expression`] syntax in a form more compatible with Rust's
usual syntax.

This means that there are a few small differences between the Python syntax
and the syntax provided in this macro:

* The pattern between the `for` and `in` tokens is a fully-fledged
  Rust pattern, which can be as simple as a simple token and as complex
  as struct destructuring.
* The expression defining the iterator after the `in` token
  must evaluate to either an `Iterator` or an `impl IntoIterator`.
* The conditional expression after the `if` token must evaluate to
  a boolean.
* You may use an `if let` clause instead of the usual `if` clause wherever
  `if` clauses are allowed. Any names introduced in the `if let` clause
  are available in any following clause.
* The expression in the beginning of the generator expression,
  the expression following the `in` token, and the expression following
  the `if` token, must all end with a semicolon (;). The only exception
  to this is the last expression following `in` or `if` in the macro,
  which may omit the trailing semicolon.

The expression replaced by the `comp!()` macro invocation is a lazy
iterator whose lifetime is bound by any references it needs to capture.
This means that it can be `.collect()`ed into any container you like.

Note though that, at least for now, all objects named in an `in` clause,
(except for the first `in` clause) must be either `Copy` or introduced by
the previous `for` or `if let` clauses. This is because the macro uses a
`move` closure (`FnOnce`) for each level of nesting, which may need to be
instantiated more than once without implicit cloning of the captured
objects.
Similarly, objects named in the "yield" expression (preceding the first
`for` clause) must be `Copy` types if they were not introduced by the final
`for` or `if let` clauses. This is because they may be used in multiple
output items.

Specifying which objects should be cloned and where may be added in the
future, but will probably require a breaking change.

This is a BNF description of the syntax used by this macro:

```bnf
comprehension ::=  expression ";" comp_for [comp_iter] [";"]
comp_iter     ::=  ";" (comp_for | comp_if | comp_if_let)
comp_for      ::=  "for" pattern "in" expression [comp_iter]
comp_if       ::=  "if" expression [comp_iter]
comp_if_let   ::=  "if" "let" pattern ("|" pattern)* "=" expression [comp_iter]
```

Just like in Python, you can nest as many `for`, `if`, and `if let`
clauses as you like.

## Examples

Simple generator expression with a conditional:

```rust
use py_comp::comp;

#[derive(Debug, PartialEq, Eq)]
struct Foo(i32);

let arr = &[Foo(11), Foo(12)];

// Notice the semicolons
let comp_vector = comp!(item; for item in arr; if item.0 % 10 == 2)
    .collect::<Vec<&Foo>>();

assert_eq!(comp_vector, vec![&Foo(12)])
```

Triple cartesian product with conditions and patterns:

```rust
use py_comp::comp;

#[derive(Debug, PartialEq, Eq)]
struct Foo(i32);

// These need to be references to arrays because of how the closures
// that the macro expands to capture their environment.
let x = &[(Foo(11), "foo"), (Foo(12), "bar")];
let y = &[Foo(21), Foo(22)];
let z = &[Foo(31), Foo(32)];

let xyz = comp!(
    (a, b, c);
    for (a, _text) in x;  // You can use any function parameter pattern.
    if a.0 % 10 == 2;
    for b in y;           // Obviously not every level requires a conditional.
    for c in z;
    if c.0 % 10 == 2;
)
.collect::<Vec<(&Foo, &Foo, &Foo)>>();

// The result vector here is short for illustration purposes
// but can be as long as long as you need it to be.
assert_eq!(xyz, vec![(&Foo(12), &Foo(21), &Foo(32)), (&Foo(12), &Foo(22), &Foo(32))])
```

Flatten a triple-nested structure + complex expression:

```rust
use py_comp::comp;

#[derive(Debug, PartialEq, Eq)]
struct Foo(i32);

let nested_3 = &[
    [
        [Foo(0), Foo(1), Foo(2)],
        [Foo(3), Foo(4), Foo(5)],
        [Foo(6), Foo(7), Foo(8)],
    ],
    [
        [Foo(9), Foo(10), Foo(11)],
        [Foo(12), Foo(13), Foo(14)],
        [Foo(15), Foo(16), Foo(17)],
    ],
    [
        [Foo(18), Foo(19), Foo(20)],
        [Foo(21), Foo(22), Foo(23)],
        [Foo(24), Foo(25), Foo(26)],
    ],
];

let nested_objects = comp!(
    {
        let inner = nested.0;
        Foo(inner + 1)
    };
    for nested_2 in nested_3;
    for nested_1 in nested_2;
    for nested in nested_1;
)
.collect::<Vec<Foo>>();

let expected_values = (1..28).map(Foo).collect::<Vec<Foo>>();

assert_eq!(expected_values, nested_objects);
```

[`generator-expression`]: https://docs.python.org/3/reference/expressions.html#generator-expressions
