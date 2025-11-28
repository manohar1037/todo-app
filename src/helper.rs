use axum::http::{self, HeaderMap, HeaderValue, Request, Uri};

pub fn create_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("myheader", HeaderValue::from_static("myvalue"));
    headers
}

/// Split on '/', drop empty segments and rejoin so repeated slashes collapse.
fn normalize_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }

    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if segs.is_empty() {
        return "/".to_string();
    }

    format!("/{}", segs.join("/"))
}

pub fn rewrite_request_uri<B>(mut req: Request<B>) -> Request<B> {
    use http::uri::PathAndQuery;

    if let Some(pq) = req.uri().path_and_query() {
        let path = pq.path();

        let needs_normalize = path.contains("//") || (path.len() > 1 && path.ends_with('/'));

        if needs_normalize {
            let new_path = normalize_path(path);

            let new_pq_string = match pq.query() {
                Some(q) => format!("{new_path}?{q}"),
                None => new_path.clone(),
            };

            if let Ok(new_pq) = PathAndQuery::from_maybe_shared(new_pq_string) {
                let mut parts = req.uri().clone().into_parts();
                parts.path_and_query = Some(new_pq);

                if let Ok(new_uri) = Uri::from_parts(parts) {
                    *req.uri_mut() = new_uri;
                }
            }
        }
    }

    req
}
