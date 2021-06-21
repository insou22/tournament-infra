CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username VARCHAR(8) UNIQUE, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name TEXT
);