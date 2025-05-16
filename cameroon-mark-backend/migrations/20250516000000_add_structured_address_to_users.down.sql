-- Add back the old location column
ALTER TABLE users
ADD COLUMN location VARCHAR(255);

-- Optional: Attempt to copy data back (might be lossy)
-- UPDATE users
-- SET location = COALESCE(address_street || ', ' || address_city, address_street, address_city) -- Example
-- WHERE address_street IS NOT NULL OR address_city IS NOT NULL;

-- Remove the new structured address columns
ALTER TABLE users
DROP COLUMN address_street,
DROP COLUMN address_city,
DROP COLUMN address_postal_code,
DROP COLUMN address_country; 