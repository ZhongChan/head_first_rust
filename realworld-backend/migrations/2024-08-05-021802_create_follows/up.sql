CREATE TABLE follows (
    follower_id INTEGER NOT NULL REFERENCES users(id),
    followee_id INTEGER NOT NULL REFERENCES users(id),
    PRIMARY KEY (follower_id, followee_id)
);

CREATE INDEX index_follows_on_follower_id ON follows (follower_id);
CREATE INDEX index_follows_on_followee_id ON follows (followee_id);
