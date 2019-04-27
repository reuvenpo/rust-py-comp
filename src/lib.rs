//! This macro implements a syntax that emulates Pythons
//! [`generator-expression`] syntax in a form more compatible with rusts
//! usual syntax.
//!
//! This means that there a few small differences between the python syntax
//! and the syntax provided in this macro:
//!
//! * The pattern between the `for` and `in` tokens is a fully-fledged
//!   rust pattern, which can be as simple as a simple token and as complex
//!   as struct destructuring.
//! * The expression defining the iterator after the `in` token
//!   must  evaluate to either an `Iterator` or an `impl IntoIterator`.
//! * The conditional expression after the `if` token must evaluate to
//!   a boolean.
//! * The expression in the beginning of the generator expression,
//!   the expression following the `in` token, and the expression following
//!   the `if` token, must all end with a semicolon (;). The only exception
//!   to this is the last expression following `in` or `if` in the macro,
//!   which may omit the trailing semicolon.
//!
//! The expression replaced by the `comp!()` macro invocation is a lazy
//! iterator whose lifetime is bound by any references it needs to capture.
//! This means that it can be `.collect()`ed into any container you like.
//!
//! This is a BNF description of the syntax used by this macro:
//!
//! ```bnf
//! comprehension ::=  expression ";" comp_for [";"]
//! comp_for      ::=  "for" pattern "in" expression [";" comp_iter]
//! comp_iter     ::=  comp_for | comp_if
//! comp_if       ::=  "if" expression [";" comp_iter]
//! ```
//!
//! Just like in Python, you can nest as many `for` and `if`
//! clauses as you like.
//!
//! # Examples
//!
//! Simple generator expression with a conditional:
//! ```
//! use py_comp::comp;
//!
//! #[derive(Debug, PartialEq, Eq)]
//! struct Foo(i32);
//!
//! let arr = &[Foo(11), Foo(12)];
//!
//! // Notice the semicolons
//! let comp_vector = comp!(item; for item in arr; if item.0 % 10 == 2)
//!     .collect::<Vec<&Foo>>();
//!
//! assert_eq!(comp_vector, vec![&Foo(12)])
//! ```
//!
//! Triple cartesian product with conditions and patterns:
//! ```
//! use py_comp::comp;
//!
//! #[derive(Debug, PartialEq, Eq)]
//! struct Foo(i32);
//!
//! // These need to be references to arrays because of how the closures
//! // that the macro expands to capture their environment.
//! let x = &[(Foo(11), "foo"), (Foo(12), "bar")];
//! let y = &[Foo(21), Foo(22)];
//! let z = &[Foo(31), Foo(32)];
//!
//! let xyz = comp!(
//!     (a, b, c);
//!     for (a, _text) in x;  // You can use any function parameter pattern.
//!     if a.0 % 10 == 2;
//!     for b in y;           // Obviously not every level requires a conditional.
//!     for c in z;
//!     if c.0 % 10 == 2;
//! )
//! .collect::<Vec<(&Foo, &Foo, &Foo)>>();
//!
//! // The result vector here is short for illustration purposes
//! // but can be as long as long as you need it to be.
//! assert_eq!(xyz, vec![(&Foo(12), &Foo(21), &Foo(32)), (&Foo(12), &Foo(22), &Foo(32))])
//! ```
//! Flatten a triple-nested structure + complex expression:
//! ```rust
//! use py_comp::comp;
//!
//! #[derive(Debug, PartialEq, Eq)]
//! struct Foo(i32);
//!
//! let nested_3 = &[
//!     [
//!         [Foo(0), Foo(1), Foo(2)],
//!         [Foo(3), Foo(4), Foo(5)],
//!         [Foo(6), Foo(7), Foo(8)],
//!     ],
//!     [
//!         [Foo(9), Foo(10), Foo(11)],
//!         [Foo(12), Foo(13), Foo(14)],
//!         [Foo(15), Foo(16), Foo(17)],
//!     ],
//!     [
//!         [Foo(18), Foo(19), Foo(20)],
//!         [Foo(21), Foo(22), Foo(23)],
//!         [Foo(24), Foo(25), Foo(26)],
//!     ],
//! ];
//!
//! let nested_objects = comp!(
//!     {
//!         let inner = nested.0;
//!         Foo(inner + 1)
//!     };
//!     for nested_2 in nested_3;
//!     for nested_1 in nested_2;
//!     for nested in nested_1;
//! )
//! .collect::<Vec<Foo>>();
//!
//! let expected_values = (1..28).map(Foo).collect::<Vec<Foo>>();
//!
//! assert_eq!(expected_values, nested_objects);
//! ```
//!
//! [`generator-expression`]: https://docs.python.org/3/reference/expressions.html#generator-expressions
//!

#![warn(clippy::all)]

use doc_comment::doctest;

doctest!("../Readme.md");

/// Check that the type of the expression passed here implements IntoIterator.
#[doc(hidden)]
#[inline(always)]
pub fn __py_comp_assert_impl_into_iter<T: IntoIterator>(_: &T) {}

/// A Python-like lazy generator-expression
///
/// For details see [module level documentation][super]
///
/// [super]: ../py_comp/index.html
#[macro_export]
macro_rules! comp {
    (
        $item_expr: expr;
        for $pattern: pat in $into_iterator: expr;
        if $condition: expr $(;)?
    ) => {{
        let into_iterator = $into_iterator;
        $crate::__py_comp_assert_impl_into_iter(&into_iterator);
        into_iterator
            .into_iter()
            .filter_map(move |$pattern|
                if $condition {
                    Some($item_expr)
                } else {
                    None
                }
            )
    }};

    (
        $item_expr: expr;
        for $pattern: pat in $into_iterator: expr $(;)?
    ) => {{
        let into_iterator = $into_iterator;
        $crate::__py_comp_assert_impl_into_iter(&into_iterator);
        into_iterator
            .into_iter()
            .map(move |$pattern| $item_expr)
    }};

    (
        $item_expr: expr;
        for $pattern: pat in $into_iterator: expr;
        if $condition: expr;
        for $($rest: tt)*
    ) => {{
        let into_iterator = $into_iterator;
        $crate::__py_comp_assert_impl_into_iter(&into_iterator);
        into_iterator
            .into_iter()
            .filter_map(move |$pattern|
                if $condition {
                    Some(comp!($item_expr; for $($rest)*))
                } else {
                    None
                }
            )
            .flatten()
    }};

    (
        $item_expr: expr;
        for $pattern: pat in $into_iterator: expr;
        for $($rest: tt)*
    ) => {{
        let into_iterator = $into_iterator;
        $crate::__py_comp_assert_impl_into_iter(&into_iterator);
        into_iterator
            .into_iter()
            .flat_map(move |$pattern|
                comp!($item_expr; for $($rest)*)
            )
    }};
}
