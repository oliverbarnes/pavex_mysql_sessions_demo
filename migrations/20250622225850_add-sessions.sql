-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    deadline TIMESTAMPTZ NOT NULL,
    state JSONB NOT NULL
);

-- Create the index on the deadline column if it doesn’t exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes
        WHERE schemaname = current_schema()
            AND tablename = 'sessions'
            AND indexname = 'idx_sessions_deadline'
    ) THEN
        CREATE INDEX idx_sessions_deadline ON sessions(deadline);
    END IF;
END $$;
