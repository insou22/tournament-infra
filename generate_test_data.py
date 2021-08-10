#!/usr/bin/python3

import sqlite3
import random
import itertools
from pprint import pprint, pformat


def expected_score(rating_a: int, rating_b: int):
    # Calculates the expected score of player A when playing against player B, given the players' ratings.
    return 1 / (1 + 10**((rating_b-rating_a) / 400))


K_FACTOR = 32


def rating_change(score: float, expected_score: float):
    # Calculates a change in rating for a player given their result and expected score.
    return K_FACTOR * (score - expected_score)


def get_latest_valid_binary_hash(user):
    for binary in user["binaries"][::-1]:
        if binary["compile_result"] == "success":
            return binary["hash"]

    return None


def get_trick_winner(turn_a, turn_b):
    if turn_a["action"][0] != turn_b["action"][0]:
        return turn_a["player_index"]
    elif int(turn_a["action"][2]) > int(turn_b["action"][2]):
        return turn_a["player_index"]
    else:
        return turn_b["player_index"]


def generate_random_hands():
    suits = ["1", "2", "3", "4"]
    ranks = ["1", "2", "3", "4", "5"]
    cards = [f"{suit} {rank}" for suit,
             rank in itertools.product(suits, ranks)]
    random.shuffle(cards)
    return cards


USERS = {
    "z5361056": {
        "display_name": "HamishWHC"
    },
    "insou22": {
        "display_name": "Zac Kologlu"
    },
    "chicken": {
        "display_name": "Chicken"
    },
    "marcchee": {
        "display_name": "Marc Chee"
    },
    "z5555555": {
        "display_name": "Generic UNSW Student"
    },
    "z5555556": {
        "display_name": "Generic UNSW Student 2"
    },
}

TOURNAMENT_ID = 1
current_timestamp = 1625787738000  # July 9th 2021, 9:42:18 AM


def get_timestamp(event: str):
    global current_timestamp
    if event == "binary_create":
        current_timestamp += random.randint(7200, 10800) * 1000
    elif event == "game_create":
        current_timestamp += random.randint(5, 10) * 1000
    elif event == "game_complete":
        current_timestamp += random.randint(5, 10) * 1000
    elif event == "turn_create":
        current_timestamp += random.randint(340, 3400)
    return current_timestamp


conn = sqlite3.connect('test.db')

for username, user in USERS.items():
    user["binaries"] = []
    user["rating"] = 1000

for i in range(0, 10):
    for username, user in USERS.items():
        result_chance = random.random()
        compile_result = "success"
        if result_chance < 0.05:
            compile_result = "failed"
        elif result_chance < 0.1:
            compile_result = "timed_out"

        binary = {
            "created_at": get_timestamp("binary_create"),
            "compile_result": compile_result,
            "compile_time_ms": random.randint(34, 340) if compile_result == "success" else None,
            "hash": f"{random.getrandbits(128):x}"[:7],
            "games": []
        }

        if compile_result == "success":
            for opponent_username, opponent in USERS.items():
                if opponent_username == username:
                    continue

                opponent_binary_hash = get_latest_valid_binary_hash(opponent)

                if opponent_binary_hash is None:
                    continue

                es = expected_score(
                    user["rating"], opponent["rating"])

                hands = generate_random_hands()

                game = {
                    "created_at": get_timestamp("game_create"),
                    "completed_at": get_timestamp("game_complete"),
                    "players": [
                        {
                            "username": username,
                            "rating_before_game": user["rating"],
                            "binary_hash": binary["hash"],
                            "score": 0,
                            "hand": hands[0:10]
                        },
                        {
                            "username": opponent_username,
                            "rating_before_game": opponent["rating"],
                            "binary_hash": opponent_binary_hash,
                            "score": 0,
                            "hand": hands[10:20]
                        }
                    ],
                    "turns": []
                }

                # index (0 or 1) of a player in `game["players"]`
                turn = random.randint(0, 1)
                for i in range(0, 20):
                    state = random.random()
                    if state < 0.05:
                        state = "timed_out"
                    elif state < 0.1:
                        state = "invalid"
                    elif state < 0.15:
                        state = "illegal"
                    else:
                        state = "legal"

                    action = random.randint(
                        0, len(game["players"][turn]["hand"]) - 1)
                    action = game["players"][turn]["hand"].pop(action)

                    game["turns"].append({
                        "player_index": turn,
                        "action": action,
                        "state": state,
                        "run_time_ms": random.randint(14, 1300),
                        "created_at": get_timestamp("turn_create"),
                        "stdout": "",
                        "stdin": "",
                        "stderr": ""
                    })

                    if i % 2 == 1:
                        turn = get_trick_winner(
                            game["turns"][-2], game["turns"][-1])
                        game["players"][turn]["score"] += 1
                    else:
                        turn = int(not bool(turn))

                if game["players"][0]["score"] == game["players"][1]["score"]:
                    game["players"][0]["points"] = 1
                    game["players"][1]["points"] = 1
                elif game["players"][0]["score"] > game["players"][1]["score"]:
                    game["players"][0]["points"] = 2
                    game["players"][1]["points"] = 0
                else:
                    game["players"][0]["points"] = 0
                    game["players"][1]["points"] = 2

                game["players"][0]["rating_change"] = int(
                    rating_change(game["players"][0]["points"] / 2, es))
                game["players"][1]["rating_change"] = int(
                    rating_change(game["players"][1]["points"] / 2, 1 - es))

                user["rating"] += game["players"][0]["rating_change"]
                opponent["rating"] += game["players"][1]["rating_change"]

                binary["games"].append(game)

        user["binaries"].append(binary)

user_inserts = [{
    "username": username,
    "tournament_id": TOURNAMENT_ID,
    "rating": user["rating"],
    "display_name": user["display_name"]
} for username, user in USERS.items()]

conn.executemany(
    """INSERT INTO users (
        username,
        display_name
    ) VALUES (
        :username,
        :display_name
    )""",
    user_inserts
)

conn.executemany(
    """INSERT INTO rankings (
        user_id,
        tournament_id,
        rating
    ) VALUES (
        (SELECT id FROM users WHERE username=:username),
        :tournament_id,
        :rating
    )""",
    user_inserts
)

binary_inserts = [{
    "username": username,
    "tournament_id": TOURNAMENT_ID,
    "created_at": binary["created_at"],
    "hash": binary["hash"],
    "compile_result": binary["compile_result"],
    "compile_time_ms": binary["compile_time_ms"],
} for username, user in USERS.items() for binary in user["binaries"]]

conn.executemany(
    """INSERT INTO binaries (
        user_id,
        tournament_id,
        created_at,
        hash,
        compile_result,
        compile_time_ms
    ) VALUES (
        (SELECT id FROM users WHERE username=:username),
        :tournament_id,
        :created_at,
        :hash,
        :compile_result,
        :compile_time_ms
    )""",
    binary_inserts
)

game_inserts = [{
    "tournament_id": TOURNAMENT_ID,
    "created_at": game["created_at"],
    "completed_at": game["completed_at"],
} for username, user in USERS.items() for binary in user["binaries"] for game in binary["games"]]

conn.executemany(
    """INSERT INTO games (
        tournament_id,
        created_at,
        completed_at
    ) VALUES (
        :tournament_id,
        :created_at,
        :completed_at
    )""",
    game_inserts
)

player_inserts = [{
    "game_created_at": game["created_at"],
    "username": player["username"],
    "binary_hash": player["binary_hash"],
    "rating_before_game": player["rating_before_game"],
    "points": player["points"],
    "rating_change": player["rating_change"]
} for username, user in USERS.items() for binary in user["binaries"] for game in binary["games"] for player in game["players"]]

conn.executemany(
    """INSERT INTO players (
        game_id,
        user_id,
        binary_id,
        rating_before_game,
        points,
        rating_change
    ) VALUES (
        (SELECT id FROM games WHERE created_at=:game_created_at),
        (SELECT id FROM users WHERE username=:username),
        (SELECT id FROM binaries WHERE hash=:binary_hash),
        :rating_before_game,
        :points,
        :rating_change
    )""",
    player_inserts
)

turn_inserts = [{
    "game_created_at": game["created_at"],
    "turn_number": turn_number,
    "username": game["players"][turn["player_index"]]["username"],
    "created_at": turn["created_at"],
    "run_time_ms": turn["run_time_ms"],
    "action": turn["action"],
    "state": turn["state"],
    "stdout": turn["stdout"],
    "stdin": turn["stdin"],
    "stderr": turn["stderr"]
} for username, user in USERS.items() for binary in user["binaries"] for game in binary["games"] for turn_number, turn in enumerate(game["turns"], start=1)]

conn.executemany(
    """INSERT INTO turns (
        game_id,
        turn_number,
        player_id,
        created_at,
        run_time_ms,
        action,
        state,
        stdout,
        stdin,
        stderr
    ) VALUES (
        (SELECT id FROM games WHERE created_at=:game_created_at),
        :turn_number,
        (SELECT id FROM players WHERE user_id=(
            SELECT id FROM users WHERE username=:username
        ) AND game_id=(
            SELECT id FROM games WHERE created_at=:game_created_at
        )),
        :created_at,
        :run_time_ms,
        :action,
        :state,
        :stdout,
        :stdin,
        :stderr
    )""",
    turn_inserts
)

conn.commit()
conn.close()

# with open("out.txt", "r+") as f:
#     f.write(pformat(USERS))
