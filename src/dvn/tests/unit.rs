use common::setup_pic;

mod common;

#[test]
fn test_hello_world() {
    let _ = setup_pic();
    // TODO
    assert_eq!(1, 1);
}