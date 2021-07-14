INSERT INTO users (username, display_name)
VALUES ('z5555555', 'HamishWHC');

INSERT INTO binaries (user_id, tournament_id, created_at, hash, time_taken_ms, timed_out)
VALUES (1, 1, 0, 'test', 1, FALSE);

INSERT INTO games (id, user_id, tournament_id, binary_id, created_at, completed_at, points, elo_before_game, elo_change)
VALUES (1, 1, 1, 1, 0, 1, 2, 1000, 10);

INSERT INTO turns (game_id, turn_number, user_id, binary_id, created_at, time_taken_ms, timed_out, legal, stdout, stderr, stdin)
VALUES (1, 1, 1, 1, 0, 1, FALSE, TRUE, '', '', '');