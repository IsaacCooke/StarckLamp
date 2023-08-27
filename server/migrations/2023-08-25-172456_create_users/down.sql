-- Drop the triggers
DROP TRIGGER IF EXISTS lamps_set_created_at ON lamps;
DROP TRIGGER IF EXISTS lamps_set_updated_at ON lamps;

-- Drop the functions
DROP FUNCTION IF EXISTS set_created_at();
DROP FUNCTION IF EXISTS set_updated_at();

DROP TABLE IF EXISTS lamps;