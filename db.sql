CREATE TABLE users (
    user_id SERIAL,
    user_name VARCHAR PRIMARY KEY,
    password VARCHAR
);
CREATE INDEX username ON users(user_name)