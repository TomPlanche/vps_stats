CREATE TABLE IF NOT EXISTS city (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    latitude REAL,
    longitude REAL
);

CREATE TRIGGER IF NOT EXISTS update_city_updated_at AFTER
UPDATE ON city FOR EACH ROW BEGIN
UPDATE city
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = NEW.id;

END;

CREATE TABLE IF NOT EXISTS event (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    referrer TEXT,
    name TEXT NOT NULL,
    collector_id TEXT NOT NULL
);

CREATE TRIGGER IF NOT EXISTS update_event_updated_at AFTER
UPDATE ON event FOR EACH ROW BEGIN
UPDATE event
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = NEW.id;

END;

CREATE TABLE IF NOT EXISTS collector (
    id TEXT PRIMARY KEY NOT NULL,
    origin TEXT NOT NULL,
    city_id INTEGER NOT NULL,
    os TEXT,
    browser TEXT,
    FOREIGN KEY (city_id) REFERENCES city (id)
);

CREATE TRIGGER IF NOT EXISTS update_collector_updated_at AFTER
UPDATE ON collector FOR EACH ROW BEGIN
UPDATE collector
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = NEW.id;

END;
