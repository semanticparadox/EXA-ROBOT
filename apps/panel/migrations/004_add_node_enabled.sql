-- Add is_enabled column to nodes table
ALTER TABLE nodes ADD COLUMN is_enabled BOOLEAN DEFAULT 1;

-- Update existing nodes to be enabled
UPDATE nodes SET is_enabled = 1;
