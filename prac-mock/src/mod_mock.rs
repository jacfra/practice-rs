use mockall::mock;

mod example {}

mock! {
    pub Example{
        fn example_fn() -> i32{
            unimplemented!()
        }
    }
}

#[test]
fn mock_struct_trait_impl() {
    let mock_data = 1;
    let mut mock = MockExample::new();
    mock.expect_example_fn().returning(|| 1);
    let mock_response = mock.example_fn();
    assert_eq!(mock_response, mock_data);
}
