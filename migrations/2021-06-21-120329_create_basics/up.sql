CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username VARCHAR(8) UNIQUE, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name VARCHAR(32)
);

CREATE TABLE games (
    id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    points INTEGER,
    elo_change INTEGER,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    tournament_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (game_id, user_id),
    PRIMARY KEY (id, user_id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    time_taken INTEGER,
    timed_out BOOLEAN,
    legal BOOLEAN,
    stdout TEXT,
    stderr TEXT,
    stdin TEXT,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (player_id) REFERENCES games (user_id),
    UNIQUE (game_id, turn_number)
);