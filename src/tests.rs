use crate::recursion::recursion;

#[test]
fn recursion_2() {
    recursion(2).unwrap();
}

#[test]
fn recursion_3() {
    recursion(3).unwrap();
}

#[test]
fn recursion_5() {
    recursion(5).unwrap();
}

#[test]
fn recursion_10() {
    recursion(10).unwrap();
}

#[test]
fn recursion_20() {
    recursion(20).unwrap();
}
