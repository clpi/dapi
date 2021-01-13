use di_api::run;

#[async_std::main]
pub async fn main() -> Result<(), tide::http::Error> {
    run().await

}
