use self::index::AddIndexRoutes;
use axum::{
    body::{self, HttpBody},
    extract::Path,
    http::{header, status, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use maud::{html, PreEscaped, DOCTYPE};
use std::{fs, io, str::FromStr};

mod index;

type Html = PreEscaped<String>;

pub fn all_routes() -> Router {
    Router::new()
        .route_index()
        .route("/static/*path", get(static_path))
        .route("/*path", get(page_not_found))
}

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    const ROOT: &'static str = "./static/";

    let file_contents = fs::read_to_string(format!("{ROOT}/{path}"));
    let file_type = mime_guess::from_path(&path).first_or_text_plain();

    match file_contents {
        Err(error) => {
            let status_code = if error.kind() == io::ErrorKind::NotFound {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            if error.kind() == io::ErrorKind::NotFound
                && (file_type == mime::TEXT_HTML || file_type == mime::TEXT_HTML_UTF_8)
            {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(body::Body::from(page_not_found(Path(path)).await.0))
                    .unwrap();
            }

            Response::builder()
                .status(status_code)
                .body(body::Body::from(error.to_string()))
                .unwrap()
        }
        Ok(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(file_type.as_ref()).unwrap(),
            )
            .body(body::Body::from(file))
            .unwrap(),
    }
}

async fn page_not_found(Path(path): Path<String>) -> Html {
    html! {
        (DOCTYPE)
        head {
            link rel="stylesheet" href="./static/styles.css";
        }
        body {
            h1 { b class="primary" { "Uh oh! "} "Looks like we couldn't find the page \"" (path) "\"..." }
        }
    }
}
