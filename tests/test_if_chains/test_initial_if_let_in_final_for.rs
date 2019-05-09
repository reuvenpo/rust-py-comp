//! Test the syntax works correctly for if-chains starting with an `if let`
//! clause in the final `for` clause.

use py_comp::comp;

#[test]
fn for_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(*a; for (a, _) in iterable; if let 2...5 = a).collect();

    assert_eq!(items, vec![2, 3, 4, 5]);
}

#[test]
fn for_if_let_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if let 2...5 = a;
        if *b >= 13 && *b <= 14
    )
    .collect();

    assert_eq!(items, vec![3, 4]);
}

#[test]
fn for_if_let_if_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if let 2...5 = a;
        if *b >= 13 && *b <= 15;
        if *b >= 14 && *b <= 14
    )
    .collect();

    assert_eq!(items, vec![4]);
}

#[test]
fn for_if_let_if_if_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if let 2...5 = a;
        if *b >= 13 && *b <= 15;
        if *b >= 13 && *b <= 14;
        if let 11...13 = b
    )
    .collect();

    assert_eq!(items, vec![3]);
}

#[test]
fn for_if_let_if_let() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a; for (a, b) in iterable; if let 2...5 = a; if let 11...14 = b
    )
    .collect();

    assert_eq!(items, vec![2, 3, 4]);
}

#[test]
fn for_if_let_if_let_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if let 2...5 = a;
        if let 11...14 = b;
        if *a >= 2 && *a <= 3
    )
    .collect();

    assert_eq!(items, vec![2, 3]);
}

#[test]
fn for_if_let_if_let_if_if() {
    let iterable = &[(1, 11), (2, 12), (3, 13), (4, 14), (5, 15)];

    let items: Vec<i32> = comp!(
        *a;
        for (a, b) in iterable;
        if let 2...5 = a;
        if let 11...14 = b;
        if *a >=2 && *a <= 3;
        if *a >=3 && *a <= 4
    )
    .collect();

    assert_eq!(items, vec![3]);
}
