CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  validated  BOOLEAN   NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');