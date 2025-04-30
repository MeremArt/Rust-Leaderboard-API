```md
# 🏆 Leaderboard API (Rust + Actix-Web + MongoDB)

A blazing fast RESTful API for managing a game leaderboard built in **Rust** using **Actix-Web**, **MongoDB**, and **Tokio** runtime.

## ✨ Features

- ✅ Add player scores
- ✅ Get all players
- ✅ Get top N players sorted by score
- ✅ Delete player by ID
- ✅ MongoDB-backed persistence
- ✅ Full JSON API
- ✅ `.env` configuration
- ✅ Modular service & route separation

---

## 🚀 Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Actix-Web](https://actix.rs/)
- [MongoDB Atlas](https://www.mongodb.com/cloud/atlas)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)
- [dotenvy](https://docs.rs/dotenvy/)

---

## 📁 Project Structure
```

src/
├── main.rs # App entry point
├── models.rs # Data models
├── routes.rs # Actix route handlers
├── services.rs # Business logic
├── state.rs # Shared MongoDB connection
├── errors.rs # Central error handling
.env # Environment variables

````

---

## 🧪 API Endpoints

### ➕ Add a Player
`POST /leaderboard`
```json
{
  "name": "Tim",
  "score": 1500
}
````

### 📋 Get All Players

`GET /leaderboard`

### 🥇 Get Top N Players

`GET /leaderboard/top/{count}`  
Example: `/leaderboard/top/5`

### ❌ Delete a Player

`DELETE /leaderboard/{id}`

---

## 🔧 Setup & Run

### 1. 📦 Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 📁 Clone this repo

```bash
git clone https://github.com/your-username/leaderboard-api.git
cd leaderboard-api
```

### 3. 📄 Set environment variables

Create a `.env` file in the root:

```env
MONGODB_URI=mongodb+srv://<username>:<password>@your-cluster.mongodb.net/?retryWrites=true&w=majority
MONGO_DB_NAME=leaderboard_db
PORT=8080
```

⚠️ Make sure `.env` is in `.gitignore`!

---

### 4. 🚀 Run the API

```bash
cargo run
```

The server starts on: `http://localhost:8080`

---

## 🧠 Example Response

```json
[
  {
    "id": "65f1d0c3b0c1f1...",
    "name": "Alice",
    "score": 1400
  },
  {
    "id": "65f1d0d5b0c1f2...",
    "name": "Bob",
    "score": 1300
  }
]
```

---

## ✅ Future Improvements

- 🔒 JWT Authentication
- 🧾 Pagination support
- 📊 Score history tracking
- 🧪 Unit + integration tests
- 🐳 Dockerized deployment

---

## 🧑‍💻 Author

Built by [@ugofranklin22](https://github.com/ugofranklin22)  
Open for feedback, contributions & collabs 🚀

---

## 📜 License

MIT © 2025

```

```
