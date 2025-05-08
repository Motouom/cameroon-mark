
# Cameroon Mark API Endpoints Documentation

This document outlines the necessary endpoints for integrating the Cameroon Mark frontend with a Rust and PostgreSQL backend.

## Base URL

```
https://api.cameroonmark.com/v1
```

## Authentication Endpoints

### Register User

```
POST /auth/register
```

**Request Body:**
```json
{
  "name": "User Name",
  "email": "user@example.com",
  "password": "securepassword",
  "role": "buyer" // or "seller"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Registration successful",
  "data": {
    "id": "user_id",
    "name": "User Name",
    "email": "user@example.com",
    "role": "buyer",
    "createdAt": "2023-05-15T10:30:00"
  },
  "token": "jwt_token_here"
}
```

### Login

```
POST /auth/login
```

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "id": "user_id",
    "name": "User Name",
    "email": "user@example.com",
    "role": "buyer",
    "createdAt": "2023-05-15T10:30:00"
  },
  "token": "jwt_token_here"
}
```

### Password Reset Request

```
POST /auth/reset-password
```

**Request Body:**
```json
{
  "email": "user@example.com"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Password reset link sent to your email"
}
```

## User Endpoints

### Get Current User

```
GET /users/me
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "user_id",
    "name": "User Name",
    "email": "user@example.com",
    "role": "buyer",
    "createdAt": "2023-05-15T10:30:00",
    "location": "Douala, Cameroon",
    "phone": "+237123456789"
  }
}
```

### Update User Profile

```
PUT /users/me
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "name": "Updated Name",
  "phone": "+237123456789",
  "location": "Douala, Cameroon"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Profile updated successfully",
  "data": {
    "id": "user_id",
    "name": "Updated Name",
    "email": "user@example.com",
    "phone": "+237123456789",
    "location": "Douala, Cameroon"
  }
}
```

### Change Password

```
PUT /users/me/password
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "currentPassword": "old_password",
  "newPassword": "new_password"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Password updated successfully"
}
```

## Products Endpoints

### Get All Products

```
GET /products
```

**Query Parameters:**
- `page` (optional): Page number for pagination (default: 1)
- `limit` (optional): Number of products per page (default: 20)
- `category` (optional): Filter by category ID
- `search` (optional): Search query
- `minPrice` (optional): Minimum price
- `maxPrice` (optional): Maximum price
- `sort` (optional): Sort option ('latest', 'price-low', 'price-high', 'rating')

**Response:**
```json
{
  "success": true,
  "data": {
    "products": [
      {
        "id": "product_id",
        "title": "Product Name",
        "description": "Product Description",
        "price": 1500.0,
        "images": ["url1", "url2"],
        "categoryId": "category_id",
        "sellerId": "seller_id",
        "sellerName": "Seller Name",
        "stock": 10,
        "location": "Douala, Cameroon",
        "createdAt": "2023-05-15T10:30:00",
        "featured": true,
        "rating": 4.5
      }
    ],
    "pagination": {
      "total": 100,
      "pages": 5,
      "currentPage": 1,
      "limit": 20
    }
  }
}
```

### Get Single Product

```
GET /products/:id
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "product_id",
    "title": "Product Name",
    "description": "Product Description",
    "price": 1500.0,
    "images": ["url1", "url2"],
    "categoryId": "category_id",
    "category": "Category Name",
    "sellerId": "seller_id",
    "seller": {
      "id": "seller_id",
      "name": "Seller Name",
      "rating": 4.7,
      "products": 15
    },
    "stock": 10,
    "location": "Douala, Cameroon",
    "createdAt": "2023-05-15T10:30:00",
    "featured": true,
    "rating": 4.5,
    "reviews": [
      {
        "id": "review_id",
        "userId": "user_id",
        "userName": "User Name",
        "rating": 5,
        "comment": "Great product!",
        "createdAt": "2023-05-20T14:30:00"
      }
    ]
  }
}
```

### Create Product

```
POST /products
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "title": "New Product",
  "description": "Product Description",
  "price": 1500.0,
  "images": ["url1", "url2"],
  "categoryId": "category_id",
  "stock": 10,
  "location": "Douala, Cameroon"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Product created successfully",
  "data": {
    "id": "new_product_id",
    "title": "New Product",
    "description": "Product Description",
    "price": 1500.0,
    "images": ["url1", "url2"],
    "categoryId": "category_id",
    "sellerId": "seller_id",
    "stock": 10,
    "location": "Douala, Cameroon",
    "createdAt": "2023-05-15T10:30:00"
  }
}
```

### Update Product

```
PUT /products/:id
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "title": "Updated Product",
  "description": "Updated Description",
  "price": 2000.0,
  "images": ["url1", "url2", "url3"],
  "categoryId": "category_id",
  "stock": 15,
  "location": "Yaoundé, Cameroon"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Product updated successfully",
  "data": {
    "id": "product_id",
    "title": "Updated Product",
    "description": "Updated Description",
    "price": 2000.0,
    "images": ["url1", "url2", "url3"],
    "categoryId": "category_id",
    "sellerId": "seller_id",
    "stock": 15,
    "location": "Yaoundé, Cameroon",
    "updatedAt": "2023-05-16T10:30:00"
  }
}
```

### Delete Product

```
DELETE /products/:id
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "message": "Product deleted successfully"
}
```

## Categories Endpoints

### Get All Categories

```
GET /categories
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "category_id",
      "name": "Category Name",
      "description": "Category Description",
      "image": "url",
      "productCount": 25
    }
  ]
}
```

## Image Upload Endpoints

### Get Pre-signed URLs for Image Upload (MinIO)

```
POST /uploads/presigned-url
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "fileTypes": ["image/jpeg", "image/png"],
  "count": 3
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "urls": [
      {
        "uploadUrl": "presigned_url_1",
        "fileUrl": "final_file_url_1"
      },
      {
        "uploadUrl": "presigned_url_2",
        "fileUrl": "final_file_url_2"
      },
      {
        "uploadUrl": "presigned_url_3",
        "fileUrl": "final_file_url_3"
      }
    ]
  }
}
```

## Cart Endpoints

### Get Cart

```
GET /cart
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "productId": "product_id",
        "product": {
          "id": "product_id",
          "title": "Product Name",
          "description": "Product Description",
          "price": 1500.0,
          "images": ["url1"],
          "sellerId": "seller_id",
          "sellerName": "Seller Name",
          "stock": 10
        },
        "quantity": 2,
        "addedAt": "2023-05-15T10:30:00"
      }
    ],
    "totalItems": 2,
    "totalPrice": 3000.0
  }
}
```

### Add Item to Cart

```
POST /cart
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "productId": "product_id",
  "quantity": 2
}
```

**Response:**
```json
{
  "success": true,
  "message": "Item added to cart",
  "data": {
    "items": [/* updated cart items */],
    "totalItems": 2,
    "totalPrice": 3000.0
  }
}
```

### Update Cart Item

```
PUT /cart/:productId
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "quantity": 3
}
```

**Response:**
```json
{
  "success": true,
  "message": "Cart item updated",
  "data": {
    "items": [/* updated cart items */],
    "totalItems": 3,
    "totalPrice": 4500.0
  }
}
```

### Remove Cart Item

```
DELETE /cart/:productId
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "message": "Item removed from cart",
  "data": {
    "items": [/* updated cart items */],
    "totalItems": 0,
    "totalPrice": 0.0
  }
}
```

### Clear Cart

```
DELETE /cart
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "message": "Cart cleared",
  "data": {
    "items": [],
    "totalItems": 0,
    "totalPrice": 0.0
  }
}
```

## Orders Endpoints

### Create Order

```
POST /orders
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "items": [
    {
      "productId": "product_id",
      "quantity": 2
    }
  ],
  "shippingAddress": {
    "fullName": "User Name",
    "addressLine1": "123 Street",
    "addressLine2": "Apt 456",
    "city": "Douala",
    "postalCode": "12345",
    "country": "Cameroon",
    "phone": "+237123456789"
  },
  "paymentMethod": "card"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Order created successfully",
  "data": {
    "orderId": "order_id",
    "totalAmount": 3000.0,
    "paymentStatus": "pending",
    "paymentLink": "payment_gateway_url"
  }
}
```

### Get Orders for User/Seller

```
GET /orders
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Query Parameters:**
- `page` (optional): Page number for pagination (default: 1)
- `limit` (optional): Number of orders per page (default: 20)
- `status` (optional): Filter by order status

**Response:**
```json
{
  "success": true,
  "data": {
    "orders": [
      {
        "id": "order_id",
        "buyerId": "buyer_id",
        "buyerName": "Buyer Name",
        "items": [
          {
            "productId": "product_id",
            "productName": "Product Name",
            "price": 1500.0,
            "quantity": 2,
            "image": "url"
          }
        ],
        "totalAmount": 3000.0,
        "status": "processing",
        "paymentStatus": "paid",
        "createdAt": "2023-05-15T10:30:00",
        "shippingAddress": {
          "fullName": "User Name",
          "addressLine1": "123 Street",
          "city": "Douala",
          "country": "Cameroon"
        }
      }
    ],
    "pagination": {
      "total": 5,
      "pages": 1,
      "currentPage": 1,
      "limit": 20
    }
  }
}
```

### Get Single Order

```
GET /orders/:id
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "order_id",
    "buyerId": "buyer_id",
    "buyerName": "Buyer Name",
    "items": [
      {
        "productId": "product_id",
        "productName": "Product Name",
        "price": 1500.0,
        "quantity": 2,
        "image": "url",
        "sellerId": "seller_id",
        "sellerName": "Seller Name"
      }
    ],
    "totalAmount": 3000.0,
    "status": "processing",
    "paymentStatus": "paid",
    "createdAt": "2023-05-15T10:30:00",
    "shippingAddress": {
      "fullName": "User Name",
      "addressLine1": "123 Street",
      "addressLine2": "Apt 456",
      "city": "Douala",
      "postalCode": "12345",
      "country": "Cameroon",
      "phone": "+237123456789"
    }
  }
}
```

### Update Order Status (Seller only)

```
PUT /orders/:id/status
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "status": "shipped"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Order status updated successfully",
  "data": {
    "id": "order_id",
    "status": "shipped",
    "updatedAt": "2023-05-16T10:30:00"
  }
}
```

## Messages Endpoints

### Get Messages for User/Seller

```
GET /messages
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "message_id",
      "senderId": "sender_id",
      "senderName": "Sender Name",
      "recipientId": "recipient_id",
      "recipientName": "Recipient Name",
      "subject": "Message Subject",
      "message": "Message content",
      "read": false,
      "createdAt": "2023-05-15T10:30:00"
    }
  ]
}
```

### Get Message Thread

```
GET /messages/:threadId
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "data": {
    "threadId": "thread_id",
    "subject": "Thread Subject",
    "messages": [
      {
        "id": "message_id",
        "senderId": "sender_id",
        "senderName": "Sender Name",
        "message": "Message content",
        "createdAt": "2023-05-15T10:30:00"
      }
    ]
  }
}
```

### Send Message

```
POST /messages
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Request Body:**
```json
{
  "recipientId": "recipient_id",
  "subject": "Message Subject",
  "message": "Message content",
  "threadId": "thread_id" // Optional, for replies
}
```

**Response:**
```json
{
  "success": true,
  "message": "Message sent successfully",
  "data": {
    "id": "message_id",
    "threadId": "thread_id",
    "createdAt": "2023-05-15T10:30:00"
  }
}
```

### Mark Message as Read

```
PUT /messages/:id/read
```

**Headers:**
```
Authorization: Bearer jwt_token_here
```

**Response:**
```json
{
  "success": true,
  "message": "Message marked as read"
}
```

## Database Schema (PostgreSQL)

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('buyer', 'seller', 'admin')),
    location VARCHAR(100),
    phone VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Categories Table

```sql
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    image_url VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Products Table

```sql
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    images TEXT[] NOT NULL,
    category_id UUID REFERENCES categories(id),
    seller_id UUID REFERENCES users(id),
    stock INTEGER NOT NULL DEFAULT 0,
    location VARCHAR(100),
    featured BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_seller ON products(seller_id);
CREATE INDEX idx_products_price ON products(price);
CREATE INDEX idx_products_featured ON products(featured);
```

### Reviews Table

```sql
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID REFERENCES products(id),
    user_id UUID REFERENCES users(id),
    rating INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(product_id, user_id)
);

CREATE INDEX idx_reviews_product ON reviews(product_id);
```

### Orders Table

```sql
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    buyer_id UUID REFERENCES users(id),
    total_amount DECIMAL(10, 2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'shipped', 'delivered', 'cancelled')),
    payment_status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (payment_status IN ('pending', 'paid', 'failed')),
    shipping_name VARCHAR(100) NOT NULL,
    shipping_address_1 VARCHAR(100) NOT NULL,
    shipping_address_2 VARCHAR(100),
    shipping_city VARCHAR(100) NOT NULL,
    shipping_postal_code VARCHAR(20),
    shipping_country VARCHAR(100) NOT NULL,
    shipping_phone VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_orders_buyer ON orders(buyer_id);
CREATE INDEX idx_orders_status ON orders(status);
```

### Order Items Table

```sql
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES orders(id),
    product_id UUID REFERENCES products(id),
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    seller_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_order_items_order ON order_items(order_id);
CREATE INDEX idx_order_items_product ON order_items(product_id);
CREATE INDEX idx_order_items_seller ON order_items(seller_id);
```

### Messages Table

```sql
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    thread_id UUID,
    sender_id UUID REFERENCES users(id),
    recipient_id UUID REFERENCES users(id),
    subject VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    read BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_messages_sender ON messages(sender_id);
CREATE INDEX idx_messages_recipient ON messages(recipient_id);
CREATE INDEX idx_messages_thread ON messages(thread_id);
```

### Cart Items Table

```sql
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    product_id UUID REFERENCES products(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, product_id)
);

CREATE INDEX idx_cart_items_user ON cart_items(user_id);
```
