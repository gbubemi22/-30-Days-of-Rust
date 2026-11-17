use testable_math::add;

#[test]
fn adds_through_the_public_api() {
    assert_eq!(add(4, 5), 9);
}
