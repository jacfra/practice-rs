use crate::{struct_mock::Bar, trait_mock::Foo};

#[mockall::automock]
impl Foo for Bar {
    fn foo(&self) -> i32 {
        1
    }
}

#[test]
fn mock_struct_trait_impl() {
    let mock_data = 1;
    let mut mock = MockBar::new();
    mock.expect_foo().returning(|| 1);
    let mock_response = mock.foo();
    assert_eq!(mock_response, mock_data)
}
