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

-- FAKE DATA FOR TESTS
-- Cities
INSERT INTO
    city (name, country, latitude, longitude)
VALUES
    ('New York', 'USA', 40.7128, -74.0060),
    ('London', 'UK', 51.5074, -0.1278),
    ('Tokyo', 'Japan', 35.6762, 139.6503),
    ('Paris', 'France', 48.8566, 2.3522),
    ('Sydney', 'Australia', -33.8688, 151.2093);

-- Collectors
INSERT INTO
    collector (id, origin, city_id, os, browser)
VALUES
    (
        'col_a1b2c3',
        'example.com',
        1,
        'Windows',
        'Chrome'
    ),
    ('col_d4e5f6', 'example.com', 1, 'macOS', 'Safari'),
    (
        'col_g7h8i9',
        'example.com',
        2,
        'Windows',
        'Firefox'
    ),
    ('col_j0k1l2', 'example.com', 2, 'iOS', 'Safari'),
    (
        'col_m3n4o5',
        'example.com',
        3,
        'Android',
        'Chrome'
    ),
    (
        'col_p6q7r8',
        'example.com',
        3,
        'Linux',
        'Firefox'
    ),
    ('col_s9t0u1', 'example.com', 4, 'Windows', 'Edge'),
    ('col_v2w3x4', 'example.com', 4, 'macOS', 'Chrome'),
    ('col_y5z6a7', 'example.com', 5, 'iOS', 'Safari'),
    (
        'col_b8c9d0',
        'example.com',
        5,
        'Android',
        'Chrome'
    );

-- Events
INSERT INTO
    event (id, url, referrer, name, collector_id, created_at)
VALUES
    (
        'evt_123',
        'https://example.com/',
        'https://google.com',
        'page_view',
        'col_a1b2c3',
        '2025-03-01 10:00:00'
    ),
    (
        'evt_124',
        'https://example.com/products',
        NULL,
        'page_view',
        'col_a1b2c3',
        '2025-03-01 10:01:00'
    ),
    (
        'evt_125',
        'https://example.com/products',
        NULL,
        'click',
        'col_a1b2c3',
        '2025-03-01 10:01:30'
    ),
    (
        'evt_126',
        'https://example.com/',
        'https://facebook.com',
        'page_view',
        'col_d4e5f6',
        '2025-03-01 11:00:00'
    ),
    (
        'evt_127',
        'https://example.com/contact',
        NULL,
        'page_view',
        'col_g7h8i9',
        '2025-03-01 12:00:00'
    ),
    (
        'evt_128',
        'https://example.com/contact',
        NULL,
        'form_submit',
        'col_g7h8i9',
        '2025-03-01 12:05:00'
    ),
    (
        'evt_129',
        'https://example.com/',
        'https://twitter.com',
        'page_view',
        'col_j0k1l2',
        '2025-03-02 09:00:00'
    ),
    (
        'evt_130',
        'https://example.com/blog',
        NULL,
        'page_view',
        'col_m3n4o5',
        '2025-03-02 10:00:00'
    ),
    (
        'evt_131',
        'https://example.com/blog/post-1',
        NULL,
        'page_view',
        'col_m3n4o5',
        '2025-03-02 10:02:00'
    ),
    (
        'evt_132',
        'https://example.com/',
        'https://bing.com',
        'page_view',
        'col_p6q7r8',
        '2025-03-02 11:00:00'
    ),
    (
        'evt_133',
        'https://example.com/products',
        NULL,
        'page_view',
        'col_s9t0u1',
        '2025-03-02 12:00:00'
    ),
    (
        'evt_134',
        'https://example.com/products/item-1',
        NULL,
        'click',
        'col_s9t0u1',
        '2025-03-02 12:01:00'
    ),
    (
        'evt_135',
        'https://example.com/',
        'https://linkedin.com',
        'page_view',
        'col_v2w3x4',
        '2025-03-02 13:00:00'
    ),
    (
        'evt_136',
        'https://example.com/about',
        NULL,
        'page_view',
        'col_v2w3x4',
        '2025-03-02 13:05:00'
    ),
    (
        'evt_137',
        'https://example.com/',
        'https://youtube.com',
        'page_view',
        'col_y5z6a7',
        '2025-03-03 09:00:00'
    ),
    (
        'evt_138',
        'https://example.com/products',
        NULL,
        'page_view',
        'col_y5z6a7',
        '2025-03-03 09:05:00'
    ),
    (
        'evt_139',
        'https://example.com/products/item-2',
        NULL,
        'click',
        'col_y5z6a7',
        '2025-03-03 09:06:00'
    ),
    (
        'evt_140',
        'https://example.com/',
        NULL,
        'page_view',
        'col_b8c9d0',
        '2025-03-03 10:00:00'
    ),
    (
        'evt_141',
        'https://example.com/contact',
        NULL,
        'page_view',
        'col_b8c9d0',
        '2025-03-03 10:05:00'
    ),
    (
        'evt_142',
        'https://example.com/contact',
        NULL,
        'form_submit',
        'col_b8c9d0',
        '2025-03-03 10:07:00'
    );
