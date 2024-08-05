CREATE TABLE article_tags (
    article_id INTEGER NOT NULL REFERENCES articles(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    PRIMARY KEY (article_id, tag_id)
);

CREATE INDEX index_article_tags_on_article_id ON article_tags (article_id);
CREATE INDEX index_article_tags_on_tag_id ON article_tags (tag_id);
