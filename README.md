# 🌤️  Weather Fetcher

A lightweight Rust-based CLI app that periodically fetches current and forecast weather data from the OpenWeather API and saves it to a local SQLite database for analysis, logging, or display.

---

## 🧭 Overview

**Weather Fetcher** is designed for headless environments like a Raspberry Pi or server, where you might want to regularly collect weather data and later display it or analyze it.
It fetches data from the [OpenWeather API](https://openweathermap.org/api) using your latitude, longitude, and API key, and stores the results locally in an SQLite database.

The app can be scheduled to run automatically every hour (or any interval) via **cron**.

---

## ⚙️ Features

- Fetches current and hourly/daily forecast data from OpenWeather.
- Stores responses in a structured SQLite database.
- Uses a simple `.toml` config file stored in your home config directory.
- Automatically creates a config template on first run if none exists.
- Easy to set up with a cron job for scheduled execution.

---

## 📦 Installation

### 1. Clone the repository

```bash
git clone git@github.com:chrcamp/weather_fetcher.git
cd weather_fetcher
```

### 2. Build a release verison

```bash
cargo build --release
```

This produces the optimized binary at:
```
target/release/weather_fetcher
```

You can **optionally** copy it somewhere in you PATH, e.g.:
```bash
sudo cp target/release/weather_fetcher /usr/local/bin/weather_fetcher
```

---

## 🧾 Configuration

On first run, Weather Fetcher will create a config file at:
```
~/.config/weather_fetcher.toml
```

You'll need to edit this file and provide your own values:
```toml
# ~/.config/weather_fetcher.toml
api_key = "YOUR_OPENWEATHER_API_KEY"
lat = 0.0000
lon = -0.0001
units = "imperial"      # or "metric"
lang = "en"
```

> [!TIP] 💡  Tip:
> You can get a free API key from [Open Weather Map](http://openweathermap.org)

---

## 🗃️ SQLite Database
Each run inserts a new weather record into the database.

Typical schema (example):

| **Column** | **Type** | **Description**                  |
| ---------- | -------- | -------------------------------- |
| dt         | INTEGER  | UNIX timestamp                   |
| temp       | REAL     | Current temperature              |
| humidity   | INTEGER  | Humidity percentage              |
| weather    | TEXT     | Short description (e.g., “Rain”) |

You can inspect the database using the SQLite CLI:
```bash
sqlite3 ~/.local/share/weather_data.db
sqlite> SELECT * FROM weather ORDER BY dt DESC LIMIT 5;
```

---

## 🕒 Running Automatically (cron)

To fetch weather data every hour, add a cron job:
```bash
crontab -e
```

Then add:
```
0 * * * * /usr/local/bin/weather_fetcher >> /home/youruser/weather_fetcher.log 2>&1
```

This runs at the top of every hour and logs output to a file.

> [!TIP] 💡  Tip:
> Test your setup manually before enabling the cron job:
> ```bash
> weather_fetcher
> ```

---

## 🧰 Development

### Run in debug mode:
```bash
cargo Run
```

### Run with release optimizations
```bash
cargo run --release
```

### Clean build artifacts
```bash
cargo clean
```

---

## 🧪 Future Plans

- Add timestamp of fetch to database tables.
- Make dates and times readable.
- Customize weather attributes to fetch (e.g. Add Chance of Precipitation)
- Integration with e-ink display (e.g., Pimoroni Inky on Raspberry Pi)

---

## 🛠️ Build With

- Rust
  - reqwest
  - serde
  - rusqlite
  - toml

---

## Author

#### Chris Campanelli
[GitHub](https://github.com/chrcamp)

---

## 📄 License

This project is licensed under the MIT License -- See the **LICENSE** file for details.
