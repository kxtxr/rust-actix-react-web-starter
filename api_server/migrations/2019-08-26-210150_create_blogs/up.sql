-- Your SQL goes here
CREATE TABLE blogs (
    id VARCHAR(64) NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL COMMENT 'blog content',
    created_at TIMESTAMP NOT NULL
);
