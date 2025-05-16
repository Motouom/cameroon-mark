-- Add new structured address columns to the users table
ALTER TABLE users
ADD COLUMN address_street TEXT,
ADD COLUMN address_city TEXT,
ADD COLUMN address_postal_code TEXT,
ADD COLUMN address_country TEXT;

-- Optional: Copy data from old 'location' to new fields if possible (example, might need adjustment)
-- This is a simple example and might not parse all 'location' strings well.
-- UPDATE users
-- SET address_street = location -- Or some parsed part of location
-- WHERE location IS NOT NULL;

-- Remove the old location column
ALTER TABLE users
DROP COLUMN location; 