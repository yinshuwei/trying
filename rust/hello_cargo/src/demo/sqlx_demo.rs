use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Postgres};

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

pub async fn run() {
    let pool = PgPool::connect("postgres://postgres:123456@localhost/todo")
        .await
        .unwrap();

    let res = sqlx::query_as::<Postgres, Todo>("SELECT id,description,done FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", res);
}
