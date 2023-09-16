//! Mt service, for watch model use `cargo watch -x 'run --bin server'`
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    let mut router =
        Router::new(
        )
        .get(
            welcome,
        );
    let url = "0.0.0.0:1234";
    let listner = TcpListener::new(url)
        .acme()
        .cache_path("temp/letsencrypt")
        .add_domain("cli.bruce.rs")
        .http01_challege(&mut router)
        .quinn(url);
    let acceptor = listner.join(TcpListener::new("0.0.0.0:80")).bind().await;
    Server::new(
        acceptor,
    )
    .serve(router)
    .await;
}

#[handler]
async fn welcome(
    res: &mut Response,
) {
    res.render(Text::Plain("Welcome to Math Tool"));
}
