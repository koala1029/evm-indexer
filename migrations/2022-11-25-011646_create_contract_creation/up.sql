CREATE TABLE contract_creations (
  hash VARCHAR PRIMARY KEY UNIQUE NOT NULL,
  block BIGINT NOT NULL,
  contract VARCHAR NOT NULL,
  chain VARCHAR NOT NULL 
)