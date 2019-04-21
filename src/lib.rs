//! This macro implements a syntax that emulates Pythons
//! [`generator-expression`] syntax in a form more compatible with rusts
//! usual syntax.
//!
//! This means that there a few small differences between the python syntax
//! and the syntax provided in this macro:
//!
//! * The expression in the beginning of the generator expression
//!   must end with a semicolon (;).
//! * The pattern between the `for` and `in` tokens is a fully-fledged
//!   rust pattern, which can be as simple as a simple token and as complex
//!   as struct destructuring.
//! * The expression defining the iterator after the `for` token
//!   (and potentially before an `if` token) must  evaluate to either an
//!   `Iterator` or an `impl IntoIterator`, and end with a semicolon (;).
//! * The conditional expression after the `if` token
//!   (and potentially before a `for` token) must evaluate to a boolean,
//!   and end with a semicolon (;).
//!
//! The expression replaced by the `comp!()` macro invocation is a lazy
//! iterator whose lifetime is bound by any references it needs to capture.
//! This means that it can be `.collect()`ed into any container you like.
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
//! let comp_vector = comp!(item; for item in arr; if item.0 % 10 == 2;)
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
//!
//! [`generator-expression`]: https://docs.python.org/3/reference/expressions.html#generator-expressions
//!

#![warn(clippy::all)]

use doc_comment::doctest;

doctest!("../Readme.md");

// Check that the type of the expression passed here implements IntoIterator.
// Hopefully this optimizes away in release builds.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_impl_into_iter {
    ($x: expr) => {
        let _ = || {
            fn assert_impl_into_iter<T>(_: T)
            where
                T: IntoIterator,
            {
            }
            assert_impl_into_iter($x);
        };
    };
}

/// A Python-like lazy generator-expression
///
/// For details see [module level documentation][super]
///
/// [super]: ../py_comp/index.html
#[macro_export(local_inner_macros)]
macro_rules! comp {
    (
        $item_expr: expr;
        for $name: pat in $iterator: expr;
        if $condition: expr;
    ) => {{
        let iter = $iterator;
        $crate::assert_impl_into_iter!(iter);
        iter
            .into_iter()
            .filter_map(move |$name|
                if $condition {
                    Some($item_expr)
                } else {
                    None
                }
            )
    }};

    (
        $item_expr: expr;
        for $name: pat in $iterator: expr;
    ) => {{
        let iter = $iterator;
        $crate::assert_impl_into_iter!(iter);
        iter
            .into_iter()
            .map(move |$name| $item_expr)
    }};

    (
        $item_expr: expr;
        for $name: pat in $iterator: expr;
        if $condition: expr;
        for $($rest: tt)*
    ) => {{
        let iter = $iterator;
        $crate::assert_impl_into_iter!(iter);
        iter
            .into_iter()
            .filter_map(move |$name|
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
        for $name: pat in $iterator: expr;
        for $($rest: tt)*
    ) => {{
        let iter = $iterator;
        $crate::assert_impl_into_iter!(iter);
        iter
            .into_iter()
            .flat_map(move |$name|
                comp!($item_expr; for $($rest)*)
            )
    }};
}
