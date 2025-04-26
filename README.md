# Rust Leaderboard API

A high-performance RESTful API for managing game leaderboards built with Rust, Actix-Web, and MongoDB.

## Features

- **Fast Performance**: Built with Rust and Actix-Web for high throughput and low latency
- **MongoDB Integration**: Persistent storage with MongoDB for reliable data management
- **RESTful API**: Clean API endpoints for leaderboard operations
- **Error Handling**: Comprehensive error handling with custom error types
- **Asynchronous Design**: Non-blocking I/O operations for optimal performance

## API Endpoints

| Method | Endpoint                   | Description                 |
| ------ | -------------------------- | --------------------------- |
| GET    | `/`                        | Health check endpoint       |
| POST   | `/leaderboard`             | Add a new player score      |
| GET    | `/leaderboard/top/{count}` | Get top N players by score  |
| DELETE | `/leaderboard/{id}`        | Delete a player score by ID |

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [MongoDB](https://www.mongodb.com/try/download/community) (4.4+)

### Setup

1. Clone the repository:

   ```
   git clone https://github.com/meremart/rust-leaderboard-api.git
   cd rust-leaderboard-api
   ```

2. Create a `.env` file in the project root:

   ```
   MONGODB_URI=mongodb://localhost:27017
   MONGODB_DATABASE=leaderboard_db
   PORT=8000
   ```

3. Build and run the project:
   ```
   cargo build
   cargo run
   ```

The server will start on `http://127.0.0.1:8000`.

## Usage Examples

### Add a new score

```bash
curl -X POST http://localhost:8000/leaderboard \
  -H "Content-Type: application/json" \
  -d '{"name": "Player1", "score": 1000}'
```

### Get top 5 players

```bash
curl http://localhost:8000/leaderboard/top/5
```

### Delete a player

```bash
curl -X DELETE http://localhost:8000/leaderboard/60d21b4667d0d8992e610c85
```

## Project Structure

```
rust-leaderboard-api/
├── src/
│   ├── main.rs         # Application entry point
│   ├── routes.rs       # API route handlers
│   ├── models.rs       # Data models
│   ├── state.rs        # Application state
│   ├── error.rs        # Error handling
│   └── db.rs           # Database connection and operations
├── Cargo.toml          # Project dependencies
├── .env                # Environment variables
└── README.md           # This file
```

## Error Handling

The API uses custom error types for consistent error responses:

- `404 Not Found`: When requested resources don't exist
- `400 Bad Request`: For invalid input data
- `500 Internal Server Error`: For server-side errors

## Development

### Dependencies

- actix-web: Web framework
- mongodb: MongoDB driver
- serde: Serialization/Deserialization
- tokio: Async runtime
- uuid: Unique ID generation
- derive_more: Derive macros for trait implementations

### Testing

Run the test suite with:

```
cargo test
```

## License

MIT License

## Contact

Create an issue in the GitHub repository for any questions or suggestions.
