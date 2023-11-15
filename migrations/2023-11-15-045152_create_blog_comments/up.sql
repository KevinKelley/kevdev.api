CREATE TABLE blog_comments (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users (id),
  post_id INTEGER NOT NULL REFERENCES blog_posts (id),
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('blog_comments');


-- ALTER TABLE posts ADD COLUMN published_at TIMESTAMP;
-- ALTER TABLE posts DROP COLUMN published_at;