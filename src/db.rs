use crate::fetch::WeatherResponse;
use rusqlite::{Connection, params};

pub fn init_db(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS current_conditions (
            dt INTEGER PRIMARY KEY,
            temp REAL,
            humidity INTEGER,
            description TEXT
        );

        CREATE TABLE IF NOT EXISTS hourly_forecast (
            dt INTEGER PRIMARY KEY,
            temp REAL,
            humidity INTEGER,
            description TEXT
        );

        CREATE TABLE IF NOT EXISTS daily_forecast (
            dt INTEGER PRIMARY KEY,
            temp_min REAL,
            temp_max REAL,
            humidity INTEGER,
            description TEXT
        );
        ",
    )?;
    Ok(())
}

pub fn insert_weather(conn: &Connection, data: &WeatherResponse) -> anyhow::Result<()> {
    let current_desc = &data.current.weather[0].description;
    conn.execute(
        "INSERT OR REPLACE INTO current_conditions (dt, temp, humidity, description)
        VALUES (?1, ?2, ?3, ?4)",
        params![
            data.current.dt,
            data.current.temp,
            data.current.humidity,
            current_desc
        ],
    )?;

    for h in data.hourly.iter().take(24) {
        let desc = &h.weather[0].description;
        conn.execute(
            "INSERT OR REPLACE INTO hourly_forecast (dt, temp, humidity, description)
            VALUES (?1, ?2, ?3, ?4)",
            params![h.dt, h.temp, h.humidity, desc],
        )?;
    }

    for d in data.daily.iter().take(5) {
        let desc = &d.weather[0].description;
        conn.execute(
            "INSERT OR REPLACE INTO daily_forecast (dt, temp_min, temp_max, humidity, description)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![d.dt, d.temp.min, d.temp.max, d.humidity, desc],
        )?;
    }

    Ok(())
}
