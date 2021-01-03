use di_api::run;

#[async_std::main]
async fn main() {
    println!("He");
    run().await??;
}
