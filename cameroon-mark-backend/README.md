# Cameroon Market Backend

A RESTful API backend for the Cameroon Market online marketplace, built with Rust and PostgreSQL.

## Technology Stack

- **Language**: Rust (latest stable version)
- **Web Framework**: Axum
- **Database**: PostgreSQL 15+
- **ORM**: SQLx
- **Authentication**: JWT tokens
- **Image Storage**: MinIO (S3-compatible)
- **Payment Processing**: Support for MTN Mobile Money, Orange Money
- **Error Handling**: Custom error types with thiserror
- **Logging**: Structured logging with tracing
- **Configuration**: Environment variables via dotenv
- **API Documentation**: OpenAPI/Swagger
- **Testing**: Unit tests and integration tests

## Project Structure

```
src/
├── main.rs (application entry point)
├── config/ (configuration handling)
├── models/ (database models)
├── routes/ (API routes)
├── handlers/ (request handlers)
├── services/ (business logic)
├── middlewares/ (authorization, etc.)
├── utils/ (helper functions)
└── errors/ (custom error types)
migrations/ (database migrations)
tests/ (unit and integration tests)
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose

### Setup

1. Clone the repository
2. Start the required services (PostgreSQL and MinIO):

```bash
docker-compose up -d
```

3. Set up the environment variables:

```bash
cp .env.example .env.local
# Edit .env.local with your configuration
```

4. Run the database migrations:

```bash
cargo run --bin migrate
```

5. Start the development server:

```bash
cargo run
```

The API will be available at `http://localhost:8081`.

## API Endpoints

### Authentication Endpoints
- POST /auth/register - User registration
- POST /auth/login - User login
- POST /auth/reset-password - Request password reset

### User Endpoints
- GET /users/me - Get current user profile
- PUT /users/me - Update user profile
- PUT /users/me/password - Change password

### Products Endpoints
- GET /products - Get all products with filtering, sorting, and pagination
- GET /products/:id - Get single product with details
- POST /products - Create new product (seller only)
- PUT /products/:id - Update product (seller only)
- DELETE /products/:id - Delete product (seller only)

### Categories Endpoints
- GET /categories - Get all categories

### Image Upload Endpoints
- POST /uploads/presigned-url - Get pre-signed URLs for S3/MinIO image upload

### Cart Endpoints
- GET /cart - Get user's cart
- POST /cart - Add item to cart
- PUT /cart/:productId - Update cart item quantity
- DELETE /cart/:productId - Remove item from cart
- DELETE /cart - Clear cart

### Orders Endpoints
- POST /orders - Create order from cart items
- GET /orders - Get orders (for buyer: their orders, for seller: orders for their products)
- GET /orders/:id - Get single order details
- PUT /orders/:id/status - Update order status (seller only)

### Messages Endpoints
- GET /messages - Get all messages for the current user
- GET /messages/:threadId - Get message thread
- POST /messages - Send message
- PUT /messages/:id/read - Mark message as read

## Development

### Running Tests

```bash
cargo test
```

### Generating API Documentation

```bash
cargo run --bin generate-docs
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
