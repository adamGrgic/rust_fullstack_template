-- Create todo_status enum type
CREATE TYPE todo_status AS ENUM ('pending', 'in_progress', 'completed', 'cancelled');

-- Drop the CHECK constraint first
ALTER TABLE todos DROP CONSTRAINT IF EXISTS todos_status_check;

-- Alter todos table to use the enum type
ALTER TABLE todos 
  ALTER COLUMN status DROP DEFAULT,
  ALTER COLUMN status TYPE todo_status USING status::todo_status,
  ALTER COLUMN status SET DEFAULT 'pending'::todo_status;
