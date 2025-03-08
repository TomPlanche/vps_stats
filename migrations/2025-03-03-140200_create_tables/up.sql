CREATE TABLE IF NOT EXISTS city (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS collector (
    id TEXT PRIMARY KEY NOT NULL,
    origin TEXT NOT NULL,
    city_id INTEGER NOT NULL,
    os TEXT,
    browser TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (city_id) REFERENCES city (id)
);

CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    referrer TEXT,
    name TEXT NOT NULL,
    collector_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (collector_id) REFERENCES collector (id)
);
