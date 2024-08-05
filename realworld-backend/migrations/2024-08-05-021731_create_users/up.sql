CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    bio TEXT,
    image VARCHAR
);

CREATE INDEX index_users_on_username ON users (username);
