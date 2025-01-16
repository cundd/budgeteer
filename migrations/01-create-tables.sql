CREATE TABLE IF NOT EXISTS transactions (
    uid INTEGER PRIMARY KEY,
    date TEXT NOT NULL,
    currency TEXT NOT NULL,
    amount REAL NOT NULL,
    type INTEGER NOT NULL,
    note TEXT NOT NULL,
    UNIQUE(date, amount, type) ON CONFLICT ABORT
) STRICT;

CREATE TABLE IF NOT EXISTS exchange_rates (
    uid INTEGER PRIMARY KEY,
    year INTEGER,
    month INTEGER,
    day INTEGER,
    base_currency TEXT NOT NULL,
    currency TEXT NOT NULL,
    rate REAL NOT NULL
) STRICT;
