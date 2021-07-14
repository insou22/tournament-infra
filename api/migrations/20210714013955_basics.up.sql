CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(8) UNIQUE NOT NULL, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name VARCHAR(32) NOT NULL
);

CREATE TABLE rankings (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    elo INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE games (
    id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    binary_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    points INTEGER,
    elo_before_game INTEGER,
    elo_change INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (id, user_id)
);

CREATE TABLE binaries (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    hash VARCHAR(256) NOT NULL,
    time_taken_ms INTEGER,
    timed_out BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY NOT NULL,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    binary_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    time_taken_ms INTEGER,
    timed_out BOOLEAN,
    legal BOOLEAN,
    stdout TEXT,
    stderr TEXT,
    stdin TEXT,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (user_id) REFERENCES games (user_id),
    FOREIGN KEY (binary_id) REFERENCES binaries (id),
    UNIQUE (game_id, turn_number)
);