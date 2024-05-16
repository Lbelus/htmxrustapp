use warp::Filter;

#[tokio::main]
async fn main() {
    let static_files = warp::fs::dir("static");
    let hello = warp::path("hello")
        .map(|| warp::reply::html("Hello, HTMX + Rust!"));
    let routes = hello.or(static_files);
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3000))
        .await;
}

