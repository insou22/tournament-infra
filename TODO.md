@insou22 here's my list of endpoints I need. I'll try and do some myself. let me know if there are any issues or questions. none of this schema is designed for multiple tournaments (my current user profile code assumes a tournament id of 1 for example), but writing code with that future change in mind would be good (current plan is to just add `/tournament/:id` in front of each endpoint when looking for data from past tournaments). implement pagination however you want.

# User Profiles
## GET /user/:username
Returns the specified user's profile (`GET /user` returns the profile of the current user, 401 if not logged in).

No request body.

Response:
```
{
    username: string,
    display_name: string,
    current_tournament_stats_summary: { // null if no ranking yet. ranking is created on submission of first binary.
        ranking: number, // current position by rating
        wins: number, // wins for this tournament
        losses: number,
        draws: number,
        average_turn_run_time_ms: number // average runtime for all this player's turns in this tournament
        rating: number // player's elo/glicko/whatever rating
    },
    current_binary: { // null if no active binary. note that a binary that fails to compile should not show up here!
        hash: string,
        created_at: number // unix timestamp for when this binary was created
        compile_result: {status: "success", time_taken_ms: number} or {status: "failure", reason: "timed_out" | "error"} or null, // note that for a user's profile, it should never be a failed binary
        stats_summary: {
            wins: number, // wins by this binary
            losses: number,
            draws: number,
            win_loss_ratio_percentage_change?: number, // percentage change from the last compiled binary (null if it is the first binary)
            average_turn_run_time_ms: number, // average runtime for turns by this binary
            average_turn_run_time_ms_percentage_change?: number // same as above, but for average turn run time.
        }
    }
}
```

## PATCH /user
Allows for partial update of user profile (for now, the only thing to be updated is their display name, so for now no need for the partial part of it yet).

Should return 403 if called with a username that is not they're own.

Request body:
```
{
    display_name: string // optional
}
```

Response: Same as `GET /user/:username`.

# Binaries
## GET /user/:username/binaries
Returns a list of binaries created by this user/current user (both compiled and failed).

No request body.

Response: List of binary objects. See `current_binary` field of `GET /user` response.

## GET /user/:username/binary/:hash
Returns binary with specified hash. Technically all binaries can be retrieved with the first endpoint, but frontend would be easier if you would make it return 404 if the wrong username is given (i.e. there is a binary with hash 123abc, and the user requests it from `insou`, except `marcchee` is the one who owns the binary, so returns 404).

No Request body.

Response: See `current_binary` field of `GET /user/:username` response.

## PUT /binaries
I don't know what the preferred way to upload a file to your game runner system is, or to Rocket, so I'll leave request details up to you.

Request: up to you.

Response: The new binary object (see `current_binary` field of `GET /user`. frontend is designed so that the compilation does not need to be done to display a binary, so you can delay that if that's useful).

# Games
## GET /games, GET /user/:username/games, GET /user/:username/binary/:hash/games
Returns a list of games within the given 'scope', latest games first. i.e. `GET /games` returns a list of the most recent games for the tournament, then `GET /user/:username/games` a list of that user's most recent games, etc.

No request body.

Response:
```
[
    {
        id: number
        created_at: number, // unix timestamp of when the game started
        completed_at: number, // unix timestamp of when it finished, null if in progress
        players: [ // one entry per player.
            {
                binary_hash: string // the hash of the binary used in this game by this 
                username: string,
                display_name: string
                elo_before_game: number, // the rating of the user before the game
                elo_change: number // the change of the user's rating after the game ended (null if in progress)
                result: either "won", "lost" or "drew"
            }
            etc.
        ]
    },

    etc.
]
```

## GET /game/:id
Returns a single game object, with detailed information on the game's turn history. Stdin, Stdout and Stderr information if the turn was played by that user.

No request body.

Response:
```
{
    id: number
    created_at: number, // unix timestamp of when the game started
    completed_at: number, // unix timestamp of when it finished, null if in progress
    players: [ // one entry per player.
        {
            binary_hash: string // the hash of the binary used in this game by this 
            username: string,
            display_name: string
            elo_before_game: number, // the rating of the user before the game
            elo_change: number // the change of the user's rating after the game ended (null if in progress)
            result: either "won", "lost" or "drew"
        }
        etc.
    ],
    turns: [
        {
            username: string,
            move: string, // string representing the move. happy to convert to a fancy look on the frontend (e.g. convert numbers to suits icons), but should be normalised (stripped whitespace, common format).
            streams: { // null if requesting user is not the one who made the move
                stdin: string,
                stdout: string,
                stderr: string
            },
            run_time: number
        },
        etc.
    ]
}
```

# Rankings
## GET /rankings
Returns a list of all players in rating order.

No request body.

Response:
```
[
    {
        username: string
        display_name: string
        rating: number
        win_loss: number
    },
    etc.
]
```