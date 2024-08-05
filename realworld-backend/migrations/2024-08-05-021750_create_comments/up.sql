CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    author_id INTEGER NOT NULL REFERENCES users(id),
    article_id INTEGER NOT NULL REFERENCES articles(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_comments_on_article_id ON comments (article_id);
CREATE INDEX index_comments_on_author_id ON comments (author_id);
