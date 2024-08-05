CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    slug VARCHAR NOT NULL UNIQUE,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    body TEXT NOT NULL,
    author_id INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_articles_on_slug ON articles (slug);
CREATE INDEX index_articles_on_author_id ON articles (author_id);
