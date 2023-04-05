#[mockall::automock]
pub trait Foo {
    fn foo(&self) -> i32;
}

#[test]
fn mock_trait() {
    let mock_data = 1;
    let mut mock = MockFoo::new();
    mock.expect_foo().returning(|| 1);
    let mock_response = mock.foo();
    assert_eq!(mock_response, mock_data)
}
