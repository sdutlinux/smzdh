CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    passworld VARCHAR(128) NOT NULL,
    salt VARCHAR(128) NOT NULL,
    created TIMESTAMP DEFAULT now()
);
