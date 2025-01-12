-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(255) NOT NULL,
  last_name TEXT NOT NULL,
  username VARCHAR(255) NOT NULL,   -- Assuming `username` is a string
  email VARCHAR(255) UNIQUE NOT NULL,  -- Unique email for each user
  user_password VARCHAR(255) NOT NULL,  -- Store hashed password here
  phone_number VARCHAR(20),  -- Optional phone number
  date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- Automatically sets the timestamp when the user is created
);
