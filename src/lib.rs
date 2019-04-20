#![warn(clippy::all)]

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

/// A Python-like lazy generator-comprehension
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
