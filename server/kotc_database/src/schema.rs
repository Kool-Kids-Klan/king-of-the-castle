table! {
    games (id) {
        id -> Int4,
        started_at -> Timestamp,
        ended_at -> Timestamp,
        winner_id -> Int4,
    }
}

table! {
    participations (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        passhash -> Varchar,
        games_played -> Int4,
        games_won -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(games -> users (winner_id));
joinable!(participations -> games (game_id));
joinable!(participations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    participations,
    users,
);
