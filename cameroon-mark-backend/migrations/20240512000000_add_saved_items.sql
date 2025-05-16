-- Add saved_items table
CREATE TABLE IF NOT EXISTS saved_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, product_id)
);

-- Create indexes for performance optimization
CREATE INDEX idx_saved_items_user_id ON saved_items(user_id);
CREATE INDEX idx_saved_items_product_id ON saved_items(product_id);