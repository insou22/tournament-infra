CREATE TABLE games (
    id INTEGER PRIMARY KEY
);

-- Going to create a separate table for players to avoid player_1_id, player_2_id etc columns in future.
CREATE TABLE players (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    won BOOLEAN, -- Can be set per player, so that in future, multiple players (e.g. in a team) can win.
    elo_change INTEGER,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (game_id, user_id)
);

CREATE TABLE turns (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    legal BOOLEAN,
    stdout TEXT,
    stderr TEXT,
    stdin TEXT,
    FOREIGN KEY (game_id) REFERENCES games (id),
    FOREIGN KEY (player_id) REFERENCES users (id),
    UNIQUE (game_id, turn_number)
);