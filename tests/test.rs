use py_comp::comp;

/// This is a stand-in for any type that does not implement Copy or Clone.
/// Using this type we can know that our implementation does not depend on
/// the implicit semantics of these traits and works for all types.
#[derive(Debug, PartialEq, Eq)]
struct Foo(i32);

/// An Iterator that is not Copy.
#[derive(Debug)]
struct UncopyableIterator {}

impl Iterator for UncopyableIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// An Iterator that is not Copy of Iterators that are not Copy.
#[derive(Debug)]
struct UncopyableIteratorOfUncopyableIterators {}

impl Iterator for UncopyableIteratorOfUncopyableIterators {
    type Item = UncopyableIterator;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[test]
fn basic_implementation_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[Foo(1), Foo(2)];

    let mut xyz1 = Vec::new();
    for a in x {
        xyz1.push(a)
    }

    #[rustfmt::skip]
        #[allow(clippy::into_iter_on_array)]
        let xyz2 =
            x
            .into_iter()
            .map(move |a| a)
            .collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn basic_implementation_with_condition_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[Foo(1), Foo(2)];

    let mut xyz1 = Vec::new();
    for a in x {
        if a.0 % 10 == 2 {
            xyz1.push(a)
        }
    }

    #[rustfmt::skip]
        #[allow(clippy::into_iter_on_array)]
        let xyz2 =
            x
            .into_iter()
            .filter(|a| a.0 % 10 == 2)
            .map(move |a| a)
            .collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn basic_implementation_cartesian_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[Foo(1), Foo(2)];
    let x = &[Foo(11), Foo(12)];
    let y = &[Foo(21), Foo(22)];
    let z = &[Foo(31), Foo(32)];

    let mut xyz1 = Vec::new();
    for a in w {
        for b in x {
            for c in y {
                for d in z {
                    xyz1.push((a, b, c, d))
                }
            }
        }
    }

    #[rustfmt::skip]
        #[allow(clippy::into_iter_on_array)]
        let xyz2 =
            w
            .into_iter()
            .flat_map(move |a| {
                x
                .into_iter()
                .flat_map(move |b| {
                    y
                    .into_iter()
                    .flat_map(move |c| {
                        z
                        .into_iter()
                        .map(move |d| {
                            (a, b, c, d)
                        })
                    })
                })
            })
            .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn basic_implementation_cartesian_with_conditions_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[Foo(1), Foo(2)];
    let x = &[Foo(11), Foo(12)];
    let y = &[Foo(21), Foo(22)];
    let z = &[Foo(31), Foo(32)];

    let mut xyz1 = Vec::new();
    for a in w.iter() {
        if a.0 % 10 == 2 {
            for b in x.iter() {
                if b.0 % 10 == 2 {
                    for c in y.iter() {
                        if c.0 % 10 == 2 {
                            for d in z.iter() {
                                if d.0 % 10 == 2 {
                                    xyz1.push((a, b, c, d))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[rustfmt::skip]
        #[allow(clippy::into_iter_on_array)]
        let xyz2 =
            w
            .into_iter()
            .filter(|a| a.0 % 10 == 2)
            .flat_map(move |a| {
                x
                .into_iter()
                .filter(|b| b.0 % 10 == 2)
                .flat_map(move |b| {
                    y
                    .into_iter()
                    .filter(|c| c.0 % 10 == 2)
                    .flat_map(move |c| {
                        z
                        .into_iter()
                        .filter(|d| d.0 % 10 == 2)
                        .map(move |d| {
                            (a, b, c, d)
                        })
                    })
                })
            })
            .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn various_forms_of_usage() {
    let x = &[Foo(1), Foo(2)];
    let y = &[[Foo(1), Foo(2)], [Foo(3), Foo(4)]];
    let z = &[
        [[Foo(1), Foo(2)], [Foo(3), Foo(4)]],
        [[Foo(1), Foo(2)], [Foo(3), Foo(4)]],
    ];
    let array_of_tuples = &[
        (Foo(1), Foo(2)),
        (Foo(3), Foo(4)),
        (Foo(1), Foo(2)),
        (Foo(3), Foo(4)),
    ];

    // importantly:
    // * trailing semicolon is optional.
    // * you can nest as many `for in` clauses as you want.
    // * you may use an `if` clause after a  `for in` clause.
    let _ = comp!(a for a in x);
    let _ = comp!(a; for a in x);
    let _ = comp!(a; for a in x;);

    let _ = comp!(a.0 + b.0 for a, b in array_of_tuples);
    let _ = comp!(a.0 + b.0 for (a, b) in array_of_tuples);
    let _ = comp!(a.0 + b.0 for (a, b) in array_of_tuples;);

    let _ = comp!(a; for a in x; if *a == Foo(123));
    let _ = comp!(a; for a in x; if *a == Foo(123););

    let _ = comp!(a; for x in y; for a in x);
    let _ = comp!(a; for x in y; for a in x;);

    let _ = comp!(a; for x in y; if x[0] == Foo(123); for a in x);
    let _ = comp!(a; for x in y; if x[0] == Foo(123); for a in x;);

    let _ = comp!(a; for x in y; for a in x; if x[0] == Foo(123));
    let _ = comp!(a; for x in y; for a in x; if x[0] == Foo(123););

    let _ = comp!(a; for y in z; for x in y; for a in x);
    let _ = comp!(a; for y in z; for x in y; for a in x;);
}

#[test]
fn comp_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[Foo(1), Foo(2)];

    let mut xyz1 = Vec::new();
    for a in x {
        xyz1.push(a)
    }

    let xyz2 = comp!(a; for a in x).collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_with_condition_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[Foo(1), Foo(2)];

    let mut xyz1 = Vec::new();
    for a in x {
        if a.0 % 10 == 2 {
            xyz1.push(a)
        }
    }

    let xyz2 = comp!(
        a;
        for a in x;
        if a.0 % 10 == 2;
    )
    .collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_with_pattern_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[(Foo(1), Foo(2)), (Foo(3), Foo(4))];

    let mut xyz1 = Vec::new();
    for (a, _b) in x {
        xyz1.push(a)
    }

    let xyz2 = comp!(a; for (a, _b) in x).collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_with_pattern_with_condition_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[(Foo(1), Foo(2)), (Foo(3), Foo(4))];

    let mut xyz1 = Vec::new();
    for (a, _b) in x {
        if a.0 % 10 == 2 {
            xyz1.push(a)
        }
    }

    let xyz2 = comp!(
        a;
        for (a, _b) in x;
        if a.0 % 10 == 2;
    )
    .collect::<Vec<&Foo>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_cartesian_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[Foo(1), Foo(2)];
    let x = &[Foo(11), Foo(12)];
    let y = &[Foo(21), Foo(22)];
    let z = &[Foo(31), Foo(32)];

    let mut xyz1 = Vec::new();
    for a in w {
        for b in x {
            for c in y {
                for d in z {
                    xyz1.push((a, b, c, d))
                }
            }
        }
    }

    let xyz2 = comp!(
        (a, b, c, d);
        for a in w;
        for b in x;
        for c in y;
        for d in z;
    )
    .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_cartesian_with_conditions_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[Foo(1), Foo(2)];
    let x = &[Foo(11), Foo(12)];
    let y = &[Foo(21), Foo(22)];
    let z = &[Foo(31), Foo(32)];

    let mut xyz1 = Vec::new();
    for a in w.iter() {
        if a.0 % 10 == 2 {
            for b in x.iter() {
                if b.0 % 10 == 2 {
                    for c in y.iter() {
                        if c.0 % 10 == 2 {
                            for d in z.iter() {
                                if d.0 % 10 == 2 {
                                    xyz1.push((a, b, c, d))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let xyz2 = comp!(
        (a, b, c, d);
        for a in w;
        if a.0 % 10 == 2;
        for b in x;
        if b.0 % 10 == 2;
        for c in y;
        if c.0 % 10 == 2;
        for d in z;
        if d.0 % 10 == 2;
    )
    .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_cartesian_with_pattern_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[(Foo(1), Foo(99)), (Foo(2), Foo(99))];
    let x = &[(Foo(11), Foo(99)), (Foo(12), Foo(99))];
    let y = &[(Foo(21), Foo(99)), (Foo(22), Foo(99))];
    let z = &[(Foo(31), Foo(99)), (Foo(32), Foo(99))];

    let mut xyz1 = Vec::new();
    for (a, _a) in w {
        for (b, _b) in x {
            for (c, _c) in y {
                for (d, _d) in z {
                    xyz1.push((a, b, c, d))
                }
            }
        }
    }

    let xyz2 = comp!(
        (a, b, c, d);
        for (a, _a) in w;
        for (b, _b) in x;
        for (c, _c) in y;
        for (d, _d) in z;
    )
    .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn comp_cartesian_with_pattern_with_conditions_4_layers() {
    // These need to be references to arrays because of how the closures
    // capture their environment
    let w = &[(Foo(1), Foo(99)), (Foo(2), Foo(99))];
    let x = &[(Foo(11), Foo(99)), (Foo(12), Foo(99))];
    let y = &[(Foo(21), Foo(99)), (Foo(22), Foo(99))];
    let z = &[(Foo(31), Foo(99)), (Foo(32), Foo(99))];

    let mut xyz1 = Vec::new();
    for (a, _a) in w.iter() {
        if a.0 % 10 == 2 {
            for (b, _b) in x.iter() {
                if b.0 % 10 == 2 {
                    for (c, _c) in y.iter() {
                        if c.0 % 10 == 2 {
                            for (d, _d) in z.iter() {
                                if d.0 % 10 == 2 {
                                    xyz1.push((a, b, c, d))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let xyz2 = comp!(
        (a, b, c, d);
        for (a, _a) in w;
        if a.0 % 10 == 2;
        for (b, _b) in x;
        if b.0 % 10 == 2;
        for (c, _c) in y;
        if c.0 % 10 == 2;
        for (d, _d) in z;
        if d.0 % 10 == 2;
    )
    .collect::<Vec<(&Foo, &Foo, &Foo, &Foo)>>();

    assert_eq!(xyz1, xyz2);
}

#[test]
fn triple_nested_structure() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
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
}

#[test]
fn uncopyable_iterator() {
    let _ = comp!(x; for x in UncopyableIterator {});
}

#[test]
fn uncopyable_iterator_of_uncopyable_iterators() {
    let _ = comp!(
        item;
        for uncopyable_iterator in UncopyableIteratorOfUncopyableIterators {};
        for item in uncopyable_iterator;
    );
}
