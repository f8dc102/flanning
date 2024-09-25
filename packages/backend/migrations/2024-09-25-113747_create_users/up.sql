CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    password_hash VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
    -- @TODO: Add more fields here
    -- @TODO: Normalization Required
    -- last_login TIMESTAMPTZ,
    -- last_login_ip INET,
    -- phone_number VARCHAR,
    -- verified BOOLEAN NOT NULL DEFAULT FALSE,
    -- level INTEGER NOT NULL DEFAULT 0,
    -- moderator BOOLEAN NOT NULL DEFAULT FALSE,
    -- admin BOOLEAN NOT NULL DEFAULT FALSE,
    -- has_two_factor BOOLEAN NOT NULL DEFAULT FALSE,
    -- phone_number_verified BOOLEAN NOT NULL DEFAULT FALSE,
    -- verified_at TIMESTAMPTZ,
    -- points INTEGER NOT NULL DEFAULT 0,
    -- suspended BOOLEAN NOT NULL DEFAULT FALSE,
    -- profile_picture_url TEXT,
    -- bio TEXT,
);
