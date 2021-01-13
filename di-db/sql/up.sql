-- TODO Handle timezones properly
-- TODO actually use this enums as types (how do they play with sqlx?)

-- CREATE TYPE status AS ENUM (
    -- 'active',
    -- 'archived',
    -- 'deleted',
    -- 'completed'
-- );

-- CREATE TYPE priority AS ENUM (
    -- 'lowest',
    -- 'low',
    -- 'medium',
    -- 'high',
    -- 'highest'
-- );

-- CREATE TYPE permission AS ENUM (
    -- 'private',
    -- 'invite_only',
    -- 'mutuals_only',
    -- 'public'
-- );

-- CREATE TYPE permission AS ENUM (
    -- 'male',
    -- 'female',
    -- 'other'
-- );

-- CREATE TYPE field_type AS ENUM (
    -- 'dropdown',
    -- 'textbox',
    -- 'enum_select_one',
    -- 'enum_select_mul',
    -- 'boolean',
    -- 'range'
-- );

CREATE TABLE Users (
    id          UUID NOT NULL PRIMARY KEY,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
    password    TEXT NOT NULL CHECK,
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS UserInfo (
    id           UUID PRIMARY KEY NOT NULL,
    uid          UUID NOT NULL REFERENCES Users(id),
    first_name   TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    last_name    TEXT CHECK (CHAR_LENGTH(first_name) < 80),
    bio          TEXT,
    img_path     TEXT,
    gender       TEXT,
    birth_date   INTEGER,
    location     TEXT,
    experience   INTEGER NOT NULL,
    user_type    INTEGER NOT NULL,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS Groups (
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    permission TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS GroupInfo (
    id UUID PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    status TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS Records (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)

CREATE TABLE IF NOT EXISTS Items (
    id UUID PRIMARY KEY NOT NULL,
    uid UUID NOT NULL REFERENCES Users(id),
    name TEXT NOT NULL CHECK (CHAR_LENGTH(name) < 80),
    status TEXT NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
)
