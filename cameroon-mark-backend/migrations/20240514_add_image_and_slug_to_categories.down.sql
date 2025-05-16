-- Drop the unique index
DROP INDEX IF EXISTS categories_slug_idx;

-- Remove the columns
ALTER TABLE categories
DROP COLUMN IF EXISTS image,
DROP COLUMN IF EXISTS slug; 