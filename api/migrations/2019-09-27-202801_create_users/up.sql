CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    email_validated BOOLEAN NOT NULL
)
