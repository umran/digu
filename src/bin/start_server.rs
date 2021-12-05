use digu::server::start;

#[tokio::main]
async fn main() {
    start().await.unwrap();
}
