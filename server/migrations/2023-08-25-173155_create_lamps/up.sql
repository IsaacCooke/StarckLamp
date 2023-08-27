CREATE TABLE IF NOT EXISTS lamps (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    red SMALLINT NOT NULL,
    green SMALLINT NOT NULL,
    blue SMALLINT NOT NULL,
    is_on BOOLEAN NOT NULL DEFAULT FALSE,
    user_id INTEGER REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION set_created_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at = NOW();
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger function to update updated_at on UPDATE
CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Creating the INSERT trigger
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_trigger
        WHERE tgname = 'lamps.set_created_at'
    ) THEN
        CREATE TRIGGER lamps_set_created_at
            BEFORE INSERT ON lamps
            FOR EACH ROW
        EXECUTE FUNCTION set_created_at();
    END IF;
end;
$$;

-- Creating the UPDATE trigger
DO $$
BEGIN
    IF NOT EXISTS(
        SELECT 1
        FROM pg_trigger
        WHERE tgname = 'lamps_set_updated_at'
    ) THEN
        CREATE TRIGGER lamps_set_updated_at
            BEFORE UPDATE ON lamps
            FOR EACH ROW
        EXECUTE FUNCTION set_updated_at();
    END IF;
end;
$$;