mod data_base;
use data_base::run_migration;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok(); // Carrega as variáveis do .env

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("Database URL: {}", database_url);
    run_migration();
}
