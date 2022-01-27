CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    salt CHAR(32) NOT NULL,
    passhash CHAR(64) NOT NULL,
    games_played INTEGER NOT NULL DEFAULT 0,
    games_won INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    started_at TIMESTAMP NOT NULL DEFAULT NOW(),
    winner_id INTEGER REFERENCES users(id)
);

CREATE TABLE participations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    game_id INTEGER REFERENCES games(id) NOT NULL
);
