DROP TABLE comments CASCADE;
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    content VARCHAR(1024) NOT NULL,
    author INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    flags bigint NOT NULL DEFAULT 0,
    created TIMESTAMP WITH TIME ZONE DEFAULT now()
);
