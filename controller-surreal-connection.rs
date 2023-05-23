use sqlx::postgres::PgConnection;
use sqlx::Connect;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Establish a connection to Surreal DB
    let connection_string = "<surreal_db_connection_string>";
    let conn = PgConnection::connect(&connection_string).await?;

    // Perform database operations using the connection

    Ok(())
}
use sqlx::query;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Establish a connection to Surreal DB

    // Execute a query
    let result = query!("SELECT * FROM my_table").fetch_all(&conn).await?;

    // Process the query result

    Ok(())
}
