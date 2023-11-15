CREATE TABLE blog_posts (
  id         SERIAL    PRIMARY KEY,
  user_id    INTEGER   NOT NULL REFERENCES users (id),
  title      VARCHAR   NOT NULL,
  body       TEXT      NOT NULL,
  published  BOOLEAN   NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('blog_posts');


-- ALTER TABLE posts ADD COLUMN published_at TIMESTAMP;
-- ALTER TABLE posts DROP COLUMN published_at;
