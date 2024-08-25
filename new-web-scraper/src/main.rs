use hyper::body::Buf;
// use hyper::{body::Body, service::service_fn, Request, Response, StatusCode};
// use std::convert::Infallible;
// use std::net::SocketAddr;
//
// mod news_scraper;
//
// async fn handle(_req: Request<dyn Body>) -> Result<Response<dyn Body>, Infallible> {
//     let url = "https://news.ycombinator.com/";
//     let mut status = StatusCode::OK;
//
//     let page = match async { Request::get(url).body() }.await {
//         Ok(b) => b,
//         Err(err) => {
//             status = err.status().unwrap_or(StatusCode::BAD_REQUEST);
//             format!("{err}")
//         }
//     };
//
//     let mut news = news_scraper::NewsScraper::new();
//     news.scrape(page);
//     let response = news.get_news();
//     let body = String::from_utf8_lossy(response.as_bytes()).to_string();
//
//     let mut res = Response::new(Body::from(body));
//     *res.status_mut() = status;
//     Ok(res)
// }
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // check if there's an environment variable for the port
//     let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
//     // parse the port into a u16
//     let port = port.parse::<u16>()?;
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], port));
//
//     println!("Listening on {}", addr);
//
//     // And a MakeService to handle each connection...
//     let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
//
//     // Then bind and serve...
//     let server = Server::bind(&addr).serve(make_service);
//
//     // And run forever...
//     Ok(server.await?)
// }
//
//
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

mod news_scraper;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let url = "https://news.ycombinator.com/";
    let mut status = StatusCode::OK;

    // Creating Hyper's client
    let client = Client::new();

    // Make an HTTP GET request using the client
    let page = match client.get(url.parse().unwrap()).await {
        Ok(response) => hyper::body::to_bytes(response.into_body()).await.unwrap(),
        Err(err) => {
            status = StatusCode::BAD_REQUEST;
            let error_message = format!("Error fetching page: {}", err);
            return Ok(Response::builder()
                .status(status)
                .body(Body::from(error_message))
                .unwrap());
        }
    };

    let mut news = news_scraper::NewsScraper::new();
    news.scrape(page.chunk());
    let response = news.get_news();
    let body = String::from_utf8_lossy(response.as_bytes()).to_string();

    let mut res = Response::new(Body::from(body));
    *res.status_mut() = status;
    Ok(res)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check if there's an environment variable for the port
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    // parse the port into a u16
    let port = port.parse::<u16>()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Listening on {}", addr);

    // A `MakeService` to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    Ok(server.await?)
}
