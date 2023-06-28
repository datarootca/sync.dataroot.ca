CREATE TABLE event
(
    eventid         INTEGER PRIMARY KEY AUTOINCREMENT,
    extid   VARCHAR NOT NULL,
    checksum   VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
