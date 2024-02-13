-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "auth_users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    password VARCHAR(100),
    otp_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    otp_verified BOOLEAN NOT NULL DEFAULT FALSE,
    otp_base32 VARCHAR(100), -- how much symbols do we need ?
    otp_auth_url VARCHAR(100),  -- how much symbols do
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);