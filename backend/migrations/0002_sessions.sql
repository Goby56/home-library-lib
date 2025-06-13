CREATE TABLE Session (
    id TEXT NOT NULL PRIMARY KEY,
    secret_hash BLOB NOT NULL, -- blob is a SQLite data type for raw binary
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- unix time (seconds)
    user INTEGER NOT NULL,
    FOREIGN KEY("user") REFERENCES "User"("id")
) STRICT;
