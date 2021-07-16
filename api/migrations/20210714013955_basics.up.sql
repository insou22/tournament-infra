CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(8) UNIQUE NOT NULL, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name VARCHAR(32) NOT NULL
);

CREATE TABLE rankings (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    rating INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (user_id, tournament_id)
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    completed_at INTEGER
);

CREATE TABLE players (
    id INTEGER PRIMARY KEY NOT NULL,
    game_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    binary_id INTEGER NOT NULL,
    rating_before_game INTEGER NOT NULL,
    points INTEGER,
    rating_change INTEGER,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (binary_id) REFERENCES binaries (id),
    UNIQUE (game_id, user_id)
);

CREATE TABLE binaries (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    hash VARCHAR(256) NOT NULL,
    compile_time_ms INTEGER,
    timed_out BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY NOT NULL,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    binary_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    run_time_ms INTEGER NOT NULL,
    timed_out BOOLEAN NOT NULL,
    action TEXT NOT NULL,
    legal BOOLEAN NOT NULL,
    stdout TEXT NOT NULL,
    stderr TEXT NOT NULL,
    stdin TEXT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (player_id) REFERENCES players (id),
    UNIQUE (game_id, turn_number)
);