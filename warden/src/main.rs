use operator::app;

#[tokio::main]
async fn main() {
    app().await.unwrap()
}
