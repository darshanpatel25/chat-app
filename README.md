# 📨 Real-Time Chat App in Rust

A real-time chat application built using **Rust**, **Actix-web**, **Kafka**, and **CockroachDB**. It supports user authentication, real-time messaging via Kafka, and persistent storage with CockroachDB.

---

## 🛠 Tech Stack

- **Rust**
- **Actix-web** – Web framework for building APIs and WebSockets
- **Apache Kafka** – For real-time message streaming
- **CockroachDB** – Distributed SQL database
- **SQLx** – Async DB layer for Rust
- **bcrypt + JWT** – Authentication

---

## 📦 Features

- 🔐 User Signup/Login (with hashed passwords)
- 🔄 JWT-based auth and middleware
- 🧵 Real-time chat using Kafka
- 💬 WebSocket integration for instant messaging
- 🗃 Persistent message and user storage in CockroachDB
- 📑 Modular structure with `models`, `controllers`, `routes`, and `services`

---

## 📁 Project Structure

```
.
├── src/
│   ├── config/           # DB and Kafka configs
│   ├── controllers/      # Business logic (Kafka producer/consumer, WebSocket)
│   ├── models/           # Data models for users/messages
│   ├── routes/           # API and WebSocket routes
│   ├── services/         # Service layers (Kafka, DB)
│   ├── utils/            # Helpers (auth, logging)
│   └── main.rs           # Entry point
├── Cargo.toml
└── README.md
```

---

## 🚀 Getting Started

### 1. Prerequisites

- Rust (nightly or stable)
- Kafka (with Zookeeper)
- CockroachDB
- Docker (optional)

---

### 2. Setup Kafka

```bash
# Start Kafka using Docker
docker-compose up -d
# Or use your own local Kafka setup
```

### 3. Setup CockroachDB

```bash
# Start single-node CockroachDB
cockroach start-single-node --insecure --listen-addr=localhost:26257
cockroach sql --insecure -e 'CREATE DATABASE chat_app;'
```

---

### 4. Create `.env`

```
DATABASE_URL=postgresql://root@localhost:26257/chat_app?sslmode=disable
KAFKA_BROKERS=localhost:9092
JWT_SECRET=your_secret_key
```

---

### 5. Run Migrations

```bash
sqlx migrate run
```

---

### 6. Run the Server

```bash
cargo run
```

---

## 📬 API Endpoints

| Method | Endpoint         | Description              |
|--------|------------------|--------------------------|
| POST   | `/api/register`  | Register a new user      |
| POST   | `/api/login`     | Login and get JWT token  |
| GET    | `/ws/chat`       | Connect WebSocket        |

---

## 🧪 Example Kafka Message Format

```json
{
  "sender_id": username,
  "receiver_id": username,
  "message": "Hello!",
  "timestamp": "2025-06-04T12:00:00Z"
}
```

---

## 📘 License

MIT License

---

## 👨‍💻 Author

**Darshan Bhensdadia**  
Backend Developer | Rust & Actix enthusiast

---
