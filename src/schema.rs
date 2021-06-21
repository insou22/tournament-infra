table! {
    games (id, user_id) {
        id -> Integer,
        user_id -> Integer,
        tournament_id -> Integer,
        created_at -> Integer,
        completed_at -> Nullable<Integer>,
        points -> Nullable<Integer>,
        elo_change -> Nullable<Integer>,
    }
}

table! {
    turns (id) {
        id -> Nullable<Integer>,
        game_id -> Integer,
        turn_number -> Integer,
        player_id -> Integer,
        created_at -> Integer,
        time_taken_ms -> Nullable<Integer>,
        timed_out -> Nullable<Bool>,
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

joinable!(games -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    turns,
    users,
);
