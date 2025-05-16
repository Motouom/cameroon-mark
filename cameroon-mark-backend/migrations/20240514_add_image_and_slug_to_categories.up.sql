-- Add image and slug columns to categories table
ALTER TABLE categories
ADD COLUMN image TEXT,
ADD COLUMN slug TEXT NOT NULL DEFAULT '';

-- Create unique index on slug
CREATE UNIQUE INDEX categories_slug_idx ON categories(slug);

-- Update existing categories with slugified names
UPDATE categories
SET slug = LOWER(REGEXP_REPLACE(name, '[^a-zA-Z0-9]+', '-', 'g'))
WHERE slug = ''; 