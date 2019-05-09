//! Test the syntax works correctly for if-chains starting with an `if`
//! clause in the final `for` clause.

use py_comp::comp;

#[test]
fn for_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(*a; for (a, _) in iterable; if *a > 1).collect();

    assert_eq!(items, vec![2, 3, 4, 5]);
}

#[test]
fn for_if_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if *a > 1;
        if let 13...14 = b
    )
    .collect();

    assert_eq!(items, vec![3, 4]);
}

#[test]
fn for_if_if_let_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if *a > 1;
        if let 13...15 = b;
        if let 14...14 = b
    )
    .collect();

    assert_eq!(items, vec![4]);
}

#[test]
fn for_if_if_let_if_let_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if *a > 1;
        if let 13...15 = b;
        if let 13...14 = b;
        if *b < 14
    )
    .collect();

    assert_eq!(items, vec![3]);
}

#[test]
fn for_if_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(*a; for (a, b) in iterable; if *a > 1; if *b < 15).collect();

    assert_eq!(items, vec![2, 3, 4]);
}

#[test]
fn for_if_if_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a; for (a, b) in iterable; if *a > 1; if *b < 15; if let 2...3 = a
    )
    .collect();

    assert_eq!(items, vec![2, 3]);
}

#[test]
fn for_if_if_if_let_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if *a > 1;
        if *b < 15;
        if let 2...3 = a;
        if let 3...4 = a
    )
    .collect();

    assert_eq!(items, vec![3]);
}
