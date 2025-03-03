CREATE TABLE IF NOT EXISTS city (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    latitude REAL,
    longitude REAL
);

CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    referrer TEXT,
    name TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    collector_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS collector (
    id TEXT PRIMARY KEY NOT NULL,
    origin TEXT NOT NULL,
    city_id INTEGER NOT NULL,
    os TEXT,
    browser TEXT,
    timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (city_id) REFERENCES city (id)
);
