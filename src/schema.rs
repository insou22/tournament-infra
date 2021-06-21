table! {
    games (id) {
        id -> Nullable<Integer>,
    }
}

table! {
    players (id) {
        id -> Nullable<Integer>,
        game_id -> Integer,
        user_id -> Integer,
        won -> Nullable<Bool>,
        elo_change -> Nullable<Integer>,
    }
}

table! {
    turns (id) {
        id -> Nullable<Integer>,
        game_id -> Integer,
        turn_number -> Integer,
        player_id -> Integer,
        legal -> Nullable<Bool>,
        stdout -> Nullable<Text>,
        stderr -> Nullable<Text>,
        stdin -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Nullable<Text>,
        display_name -> Nullable<Text>,
    }
}

joinable!(players -> games (game_id));
joinable!(players -> users (user_id));
joinable!(turns -> games (game_id));
joinable!(turns -> users (player_id));

allow_tables_to_appear_in_same_query!(
    games,
    players,
    turns,
    users,
);
