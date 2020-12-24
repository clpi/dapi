#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dsrv::api::Api::new().run().await
}
