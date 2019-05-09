//! Test the syntax works correctly for if-chains starting with an `if`
//! clause in a non final `for` clause.

use py_comp::comp;

#[test]
fn for_if_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, _) in iterable1;
        if *a > 1;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(2, 11), (3, 11), (4, 11), (5, 11)]);
}

#[test]
fn for_if_if_let_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1;
        if let 13...14 = b;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(3, 11), (4, 11)]);
}

#[test]
fn for_if_if_let_if_let_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1;
        if let 13...15 = b;
        if let 14...14 = b;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(4, 11)]);
}

#[test]
fn for_if_if_let_if_let_if_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1;
        if let 13...15 = b;
        if let 13...14 = b;
        if *b < 14;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(3, 11)]);
}

#[test]
fn for_if_if_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1;
        if *b < 15;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(2, 11), (3, 11), (4, 11)]);
}

#[test]
fn for_if_if_if_let_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1; if *b < 15;
        if let 2...3 = a;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(2, 11), (3, 11)]);
}

#[test]
fn for_if_if_if_let_if_let_for() {
    let iterable1 = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];
    let iterable2 = &[(1, 11)];

    let items: Vec<(i32, i32)> = comp!(
        (*a, *x);
        for (a, b) in iterable1;
        if *a > 1;
        if *b < 15;
        if let 2...3 = a;
        if let 3...4 = a;
        for (_, x) in iterable2
    )
    .collect();

    assert_eq!(items, vec![(3, 11)]);
}
