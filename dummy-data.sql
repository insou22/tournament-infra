INSERT INTO users (username, display_name)
VALUES ('z5555555', 'HamishWHC');

INSERT INTO users (username, display_name)
VALUES ('z5555556', 'Chicken');

INSERT INTO binaries (user_id, tournament_id, created_at, hash, compile_time_ms, timed_out)
VALUES (1, 1, 0, 'test', 1, FALSE);
INSERT INTO binaries (user_id, tournament_id, created_at, hash, compile_time_ms, timed_out)
VALUES (2, 1, 0, 'test2', 1, FALSE);

INSERT INTO games (tournament_id, created_at, completed_at)
VALUES (1, 0, 1);

INSERT INTO players (game_id, user_id, binary_id, points, rating_before_game, rating_change)
VALUES (1, 1, 1, 2, 1000, 10);
INSERT INTO players (game_id, user_id, binary_id, points, rating_before_game, rating_change)
VALUES (1, 2, 2, 0, 1000, -10);

INSERT INTO turns (game_id, turn_number, player_id, user_id, binary_id, created_at, run_time_ms, action, timed_out, legal, stdout, stderr, stdin)
VALUES (1, 1, 1, 1, 1, 0, 1, 'Ace of Hearts', FALSE, TRUE, '', '', '');
INSERT INTO turns (game_id, turn_number, player_id, user_id, binary_id, created_at, run_time_ms, action, timed_out, legal, stdout, stderr, stdin)
VALUES (1, 2, 2, 2, 2, 0, 1, '3 of Hearts', FALSE, TRUE, '', '', '');

INSERT INTO rankings (user_id, tournament_id, rating)
VALUES (1, 1, 1010);
INSERT INTO rankings (user_id, tournament_id, rating)
VALUES (2, 1, 990);