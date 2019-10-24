-- Your SQL goes here
CREATE TABLE products (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  user_id INTEGER,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);