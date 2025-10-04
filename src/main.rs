mod config;
mod db;
mod fetch;

use anyhow::Result;
use rusqlite::Connection;

fn main() -> Result<()> {
    // Load configuration (creates template if missing)
    let cfg = config::load_config();

    // Get the SQLite DB path
    let db_path = config::get_db_path();

    println!("Using DB at: {}", db_path.display());

    // Get weather
    let weather = fetch::fetch_weather(&cfg)?;

    // Connect to the SQLite database (creates file if it doesn't exist)
    let conn = Connection::open(&db_path)?;
    db::init_db(&conn)?;
    db::insert_weather(&conn, &weather)?;

    println!("Weather data stored successfully!");
    Ok(())
}
