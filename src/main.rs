use mobc_postgres::mobc::Pool;
use mobc_postgres::tokio_postgres::{Config, NoTls, Row};
use mobc_postgres::PgConnectionManager;
use std::env;
use std::str::FromStr;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//To do data structure
//
#[derive(Debug)]
struct Todo {
    id: i32,
    name: String,
}

fn row_to_todo(row: &Row) -> Todo {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    Todo { id, name }
}

async fn fetch_to_dos(db_pool: &Pool<PgConnectionManager<NoTls>>) -> Result<Vec<Todo>> {
    let client = db_pool.get().await.unwrap();
    let rows = client
        .query("SELECT id, name from todo", &[])
        .await
        .unwrap();
    Ok(rows.iter().map(|r| row_to_todo(&r)).collect())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = Config::from_str(&env::var("DATABASE_URL").unwrap()).unwrap();
    let manager = PgConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_open(20).build(manager);
    let pool = pool.clone();
    let r: Vec<Todo> = fetch_to_dos(&pool).await.unwrap();
    println!("length is {}", r.len());
    for i in r.iter() {
        println!("first name is {:?}", i);
    }
}
