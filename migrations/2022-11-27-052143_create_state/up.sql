CREATE TABLE state (
  chain VARCHAR PRIMARY KEY UNIQUE NOT NULL, 
  blocks BIGINT NOT NULL
)