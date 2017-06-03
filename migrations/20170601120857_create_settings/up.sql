CREATE TABLE settings (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  key VARCHAR NOT NULL,
  value VARCHAR NOT NULL
)
