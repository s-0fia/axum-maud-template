use super::Html;
use axum::{routing::get, Router};
use maud::{html, DOCTYPE};

pub trait AddIndexRoutes {
    fn route_index(self) -> Self;
}

impl AddIndexRoutes for Router {
    fn route_index(self) -> Self {
        self.route("/", get(index))
            .route("/index", get(index))
            .route("/index.html", get(index))
    }
}

pub async fn index() -> Html {
    html! {
        (DOCTYPE)
        head {
            script src="https://unpkg.com/htmx.org@1.9.10" { }
            link rel="stylesheet" href="./static/styles.css";
        }
        body {
            h1 {
                "Hello, world!"
            }
        }
    }
}
