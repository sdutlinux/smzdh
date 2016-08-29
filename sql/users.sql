DROP TABLE users CASCADE;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(128) NOT NULL,
    salt VARCHAR(128) NOT NULL,
    flags bigint NOT NULL DEFAULT 0,
    created TIMESTAMP with time zone DEFAULT now()
);
