use axum::http::{self, HeaderMap, HeaderValue, Request};

pub fn create_header()->HeaderMap{
    let mut headers = HeaderMap::new();
    headers.insert("myheader", HeaderValue::from_static("myvalue"));
    headers
}



pub fn rewrite_request_uri<B>(mut req: Request<B>) -> Request<B> {
    use http::uri::PathAndQuery;

    if let Some(pq) = req.uri().path_and_query() {
        let path = pq.path();

        if path.len() > 1 && path.ends_with('/') {
            let new_path = &path[..path.len() - 1];

            let new_pq = if let Some(q) = pq.query() {
                PathAndQuery::from_maybe_shared(format!("{new_path}?{q}")).unwrap()
            } else {
                PathAndQuery::from_maybe_shared(new_path.to_string()).unwrap()
            };
            let mut parts = req.uri().clone().into_parts();
            parts.path_and_query = Some(new_pq);
            let new_uri = http::Uri::from_parts(parts).expect("valid uri");
            *req.uri_mut() = new_uri;
        }
    }
    req
}
