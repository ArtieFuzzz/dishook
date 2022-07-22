use warp::{http::{Response, response::Builder}, Reply};


fn base() -> Builder {
  return Response::builder()
    .header("X-Powered-By", "ArtieFuzzz");
}
/// Returns an OK response
pub fn success() -> impl Reply {
    return base().status(200).body("OK").unwrap()
}
