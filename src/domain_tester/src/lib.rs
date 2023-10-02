//1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

//Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn test(url: String) -> String {
    let request_headers = vec![];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(1_024), //optional for request
        method: HttpMethod::GET,
        headers: request_headers,
        body: None,      //optional for request
        transform: None, //optional for request
    };

    match http_request(request).await {

        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

            str_body
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            message
        }
    }
}
