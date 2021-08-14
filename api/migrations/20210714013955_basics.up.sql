CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(8) UNIQUE NOT NULL, -- For now, this is only zIDs, so we'll restrict to 8 characters.
    display_name VARCHAR(32) NOT NULL
);

CREATE TABLE rankings (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    rating_mu REAL NOT NULL,
    rating_sigma REAL NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (user_id, tournament_id)
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    completed_at INTEGER NOT NULL
);

CREATE TABLE players (
    id INTEGER PRIMARY KEY NOT NULL,
    game_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    binary_id INTEGER NOT NULL,
    rating_mu_before_game REAL NOT NULL,
    rating_sigma_before_game REAL NOT NULL,
    rating_mu_change REAL NOT NULL,
    rating_sigma_change REAL NOT NULL,
    points INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (user_id) REFERENCES users (id), -- Technically redundant. Could join via binaries table.
    FOREIGN KEY (binary_id) REFERENCES binaries (id),
    UNIQUE (game_id, user_id)
);

CREATE TABLE binaries (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    tournament_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    hash VARCHAR(256) NOT NULL,
    compile_result TEXT NOT NULL CHECK (compile_result IN ('not_compiled', 'failed', 'timed_out', 'success')),
    compile_time_ms INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY NOT NULL,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    run_time_ms INTEGER NOT NULL,
    action TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('legal', 'illegal', 'invalid', 'timed_out')),
    stdout TEXT NOT NULL,
    stderr TEXT NOT NULL,
    stdin TEXT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (player_id) REFERENCES players (id),
    UNIQUE (game_id, turn_number)
);