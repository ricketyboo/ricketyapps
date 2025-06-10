use db::get_pool;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    init().await
}

async fn init() {
    // note: dotenvy .16+ will change how this works, using EnvLoader
    dotenv().expect(".env file not found");
    let pool = get_pool().await.expect("Oh no, pool is dead.");
    print!("Successful connection");
}
