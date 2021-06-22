table! {
    binaries (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        tournament_id -> Integer,
        created_at -> Integer,
        time_taken_ms -> Nullable<Integer>,
        timed_out -> Nullable<Bool>,
    }
}

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
    rankings (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        tournament_id -> Integer,
        elo -> Nullable<Integer>,
    }
}

table! {
    turns (id) {
        id -> Nullable<Integer>,
        game_id -> Integer,
        turn_number -> Integer,
        player_id -> Integer,
        binary_id -> Integer,
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

joinable!(binaries -> users (user_id));
joinable!(games -> users (user_id));
joinable!(rankings -> users (user_id));
joinable!(turns -> binaries (binary_id));

allow_tables_to_appear_in_same_query!(
    binaries,
    games,
    rankings,
    turns,
    users,
);