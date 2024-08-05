CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE INDEX index_tags_on_name ON tags (name);
