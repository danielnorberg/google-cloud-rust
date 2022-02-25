use tonic::{IntoRequest, IntoStreamingRequest, Request};
use tonic::codegen::futures_core::Stream;

pub fn create_request<T>(param_string: String, into_request: impl IntoRequest<T>) -> Request<T> {
    let mut request = into_request.into_request();
    let target = request.metadata_mut();
    if !param_string.is_empty() {
        target.append("x-goog-request-params", param_string.parse().unwrap());
    }
    request
}