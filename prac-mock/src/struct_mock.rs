pub struct Bar {}

#[mockall::automock]
impl Bar {
    fn baz(&self) -> u32 {
        1
    }
}

#[test]
fn mock_struct_impl() {
    let mock_data = 1;
    let mut mock = MockBar::new();
    mock.expect_baz().returning(|| 1);
    let mock_response = mock.baz();
    assert_eq!(mock_response, mock_data)
}
