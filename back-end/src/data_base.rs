use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::fs;
use std::env;
use dotenv::dotenv;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
fn execute_sql_file(conn: &mut PgConnection, file_path: &str) {
    let sql = fs::read_to_string(file_path)
        .expect(&format!("Erro ao ler o arquivo {}", file_path));

    diesel::sql_query(sql)
        .execute(conn)
        .expect(&format!("Erro ao executar SQL no arquivo {}", file_path));
}

fn run_migration() {
    let mut conn = establish_connection();
    
    // Execute a migração UP
    execute_sql_file(&mut conn, "migrations/tables/up.sql");
    execute_sql_file(&mut conn,  "migrations/tables/down.sql");
    // Se precisar rodar a migração DOWN, basta executar a função abaixo
    // execute_sql_file(&conn, "migrations/create_users_table/down.sql");
}


pub fn db_op(){

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("Database URL: {}", database_url);
    run_migration();
}
