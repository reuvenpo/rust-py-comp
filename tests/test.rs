use py_comp::comp;

/// This is a stand-in for any type that does not implement Copy or Clone.
/// Using this type we can know that our implementation does not depend on
/// the implicit semantics of these traits and works for all types.
#[derive(Debug, PartialEq, Eq)]
struct Foo(i32);

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
fn comp_1_layer() {
    // This needs to be a reference to an array because of how the closures
    // capture their environment
    let x = &[Foo(1), Foo(2)];

    let mut xyz1 = Vec::new();
    for a in x {
        xyz1.push(a)
    }

    let xyz2 = comp!(a; for a in x;).collect::<Vec<&Foo>>();

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

    let xyz2 = comp!(a; for (a, _b) in x;).collect::<Vec<&Foo>>();

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
    let obj_s_s_s = &[
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
    let objects = comp!(
        obj;
        for obj_s_s in obj_s_s_s;
        for obj_s in obj_s_s;
        for obj in obj_s;
    )
    .collect::<Vec<&Foo>>();

    let values = (0..27).map(Foo).collect::<Vec<_>>();
    let value_references = values.iter().collect::<Vec<_>>();

    assert_eq!(value_references, objects);
}
