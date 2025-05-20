Absolutely! Here's an updated `README.md` for your [Rust-Leaderboard-API](https://github.com/MeremArt/Rust-Leaderboard-API.git) that includes:

- Project description
- All endpoints (including newly added **Sign Up** & **Login**)
- Environment setup
- TLS notes
- Build/run instructions
- Example payloads

---

## 📘 `README.md` (Updated)

````markdown
# 🏆 Rust Leaderboard API

A secure, fast, and extensible **Leaderboard API** built using **Actix Web**, **MongoDB**, and **JWT authentication**. Includes HTTPS support via `rustls`.

---

## ✨ Features

- ✅ Add player scores
- 📊 Get top scores
- 📋 Get all scores
- ❌ Delete a score
- 👤 User Sign Up & Login with password hashing (Argon2 via `argonautica`)
- 🔐 JWT token-based authentication (HMAC-SHA256)
- 🔒 HTTPS with `rustls`
- 🌐 CORS-enabled for frontend access

---

## 📦 Tech Stack

- Rust (2024 edition)
- Actix Web
- MongoDB (async)
- JWT (`jwt`, `hmac`, `sha2`)
- Argon2 password hashing (`argonautica`)
- TLS via `rustls`
- dotenv config support

---

## 🚀 API Endpoints

### 👤 Auth

#### 🔐 POST `/signup`

Register a new user.

```json
{
  "username": "alice",
  "password": "secret123"
}
```
````

#### 🔐 POST `/login`

Authenticate user and get JWT token.

```json
{
  "username": "alice",
  "password": "secret123"
}
```

✅ Response:

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

---

### 🏆 Leaderboard

> ⚠️ All endpoints below are **public** by default, but can be protected with JWT middleware.

#### 📥 POST `/leaderboard`

Add a new player score.

```json
{
  "name": "bob",
  "score": 950
}
```

#### 📤 GET `/leaderboard`

Fetch all players.

#### 🥇 GET `/leaderboard/top/{count}`

Get top N scores, e.g. `/leaderboard/top/5`

#### ❌ DELETE `/leaderboard/{id}`

Delete a player's score by MongoDB ObjectId.

---

## ⚙️ Environment Variables (`.env`)

```env
MONGO_URI=mongodb://localhost:27017
MONGO_DB_NAME=leaderboard_db
JWT_SECRET=super_secret_key
PORT=8443
```

---

## 🔐 TLS (HTTPS) Setup

Generate self-signed certs (for local dev):

```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

Make sure `key.pem` and `cert.pem` are in the project root.

---

## 🛠️ Run the Project

```bash
cargo run
```

Or for hot reloading (with [cargo-watch](https://github.com/watchexec/cargo-watch)):

```bash
cargo watch -x run
```

---

## 📬 Example Requests (with `curl`)

### Sign Up

```bash
curl -X POST http://localhost:8443/signup \
-H "Content-Type: application/json" \
-d '{"username": "test", "password": "pass123"}'
```

### Login

```bash
curl -X POST http://localhost:8443/login \
-H "Content-Type: application/json" \
-d '{"username": "test", "password": "pass123"}'
```

---

## 🔒 Todo (Optional Enhancements)

- [ ] Protect leaderboard routes with JWT middleware
- [ ] Add role-based access
- [ ] Add pagination
- [ ] Add score update endpoint
- [ ] Dockerize deployment
- [ ] Unit & integration tests

---

## 👨‍💻 Author

Developed by [MeremArt](https://github.com/MeremArt) using 💪 Rust, MongoDB & Actix.

---

## 📄 License

MIT

```


```
