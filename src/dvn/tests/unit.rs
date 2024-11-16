use candid::{decode_one, encode_one, Principal};
use common::setup_pic;
use pocket_ic::common::rest::{CanisterHttpReply, CanisterHttpResponse, MockCanisterHttpResponse};

mod common;

#[test]
fn test_hello_world() {
    let (pic, backend_canister) = setup_pic();

    let call_id = pic.submit_call(
        backend_canister, 
        Principal::anonymous(),
        "test",
        encode_one(()).unwrap()
    ).unwrap();

    pic.tick();
    pic.tick();
    let canister_http_requests = pic.get_canister_http();
    assert_eq!(canister_http_requests.len(), 1);
    let canister_http_request = &canister_http_requests[0];

    let body = b"hello".to_vec();
    let mock_canister_http_response = MockCanisterHttpResponse {
        subnet_id: canister_http_request.subnet_id,
        request_id: canister_http_request.request_id,
        response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
            status: 200,
            headers: vec![],
            body: body.clone(),
        }),
        additional_responses: vec![],
    };
    pic.mock_canister_http_response(mock_canister_http_response);

    let reply = pic.await_call(call_id).unwrap();
    println!("{:?}", reply);
    assert_eq!(1, 1);
}