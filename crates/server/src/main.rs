use miniserve::{Content, Request, Response};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(_req: Request) -> Response {
    Ok(Content::Json(String::from(
        "{\"messages\": [\"Hello world!\", \"And how does that make you feel?\"]}",
    )))
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
