use trust::add;
use trust::divide;

#[test]
fn test_add() {
    assert_eq!(add(1, 1), 2)
}

#[test]
fn test_divide() {
    assert_eq!(divide(4, 2), 2)
}
