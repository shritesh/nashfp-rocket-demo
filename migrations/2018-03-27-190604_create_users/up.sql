CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  is_admin BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO users (username, is_admin) VALUES ('shritesh', 't');
INSERT INTO users (username) VALUES ('tesh');