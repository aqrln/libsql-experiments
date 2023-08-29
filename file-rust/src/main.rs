use libsql::v2::Database;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let db = Database::open("dev.db")?;
    let conn = db.connect().await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id    INTEGER PRIMARY KEY,
            email TEXT
         );",
        (),
    )
    .await?;

    conn.execute(
        "INSERT INTO user (email)
         VALUES ('user@example.com');",
        (),
    )
    .await?;

    let mut result = conn.query("SELECT * FROM user", ()).await?;
    println!("{}", common::json::to_string(&mut result)?);

    Ok(())
}
