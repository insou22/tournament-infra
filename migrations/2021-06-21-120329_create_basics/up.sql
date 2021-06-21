CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username VARCHAR(8) UNIQUE, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name VARCHAR(32)
);

CREATE TABLE games (
    id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    points INTEGER,
    elo_change INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (id, user_id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    time_taken_ms INTEGER,
    timed_out BOOLEAN,
    legal BOOLEAN,
    stdout TEXT,
    stderr TEXT,
    stdin TEXT,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (player_id) REFERENCES games (user_id),
    UNIQUE (game_id, turn_number)
);