use warp::{http::Response, Reply};

/// Returns an OK response
pub fn success() -> impl Reply {
    return Response::builder().status(200).body("OK").unwrap()
}
