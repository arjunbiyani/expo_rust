-- Your SQL goes here
CREATE TABLE Users (
   id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL ,
  username VARCHAR NOT NULL,
   active BOOLEAN NOT NULL DEFAULT '0'
)