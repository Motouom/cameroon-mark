-- Change images column type from TEXT[] to JSONB
ALTER TABLE products 
    ALTER COLUMN images TYPE JSONB 
    USING jsonb_build_array(images); 