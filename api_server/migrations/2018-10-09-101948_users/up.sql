-- Your SQL goes here
CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  hash VARCHAR(122) NOT NULL COMMENT 'argon hash',
  created_at TIMESTAMP NOT NULL
);
