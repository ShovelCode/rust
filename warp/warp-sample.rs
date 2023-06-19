use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let route = warp::any()
        .map(|| "Hello, World!");

    println!("Server started at http://localhost:3030");
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
