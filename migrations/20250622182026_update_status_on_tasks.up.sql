ALTER TABLE tasks
    ADD COLUMN completed bool GENERATED ALWAYS AS ( completed_at IS NOT NULL ) STORED;