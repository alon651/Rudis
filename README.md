# 🦀 Rudis – A Minimal Redis Clone in Rust

**Rudis** is a multithreaded, in-memory key-value store built in pure Rust, inspired by Redis. It supports a subset of Redis commands over TCP using the RESP protocol and implements both active and passive key expiration strategies.

This project demonstrates systems-level development in Rust, including protocol parsing, concurrency, and custom memory management.

---

## 🚀 Features

- ⚙️ Redis-style command support:
  - `SET`, `GET`, `DEL`, `MGET`, `ECHO`
- 🔌 **TCP Networking** with RESP (Redis Serialization Protocol)
- 🧠 **Passive Expiry**: Keys expire when accessed after TTL
- 🔥 **Active Expiry**: Background thread purges expired keys periodically
- 🧵 **Multithreaded**: Handles multiple clients concurrently using `std::thread`
- 🧼 Lightweight: No async runtime (`tokio`) or persistence — just fast and focused

---

## 📡 How It Works

- Server listens for incoming TCP connections on a specified port
- Each connection is handled in a separate thread
- RESP commands are parsed and executed in a shared key-value store
- Expiration logic is applied transparently to all keys

---

## 📦 Example Usage

```bash
$ cargo run
Rudis listening on 127.0.0.1:6379...

# Then in redis-cli (or netcat / telnet)
> SET name "Rudis" EX 10
OK
> GET name
"Rudis"
> SET counter 5
OK
> MGET name counter
1) "Rudis"
2) "5"
```

---

## 🧪 Commands Supported

| Command | Description |
|--------|-------------|
| `SET key value [EX seconds]` | Set key to value with optional TTL |
| `GET key` | Get value of a key |
| `DEL key` | Delete a key |
| `MGET key1 key2 ...` | Multi-get |
| `ECHO message` | Echo back a string |

---

## 🕑 Key Expiry

- **Passive Expiry**: Expired keys are removed when accessed
- **Active Expiry**: A background thread periodically scans and removes expired keys

---

## 🧰 Built With

- 🦀 Rust (safe, fast systems programming)
- 🔌 `std::net::TcpListener` / `TcpStream` for networking
- 🧠 Custom RESP protocol parser
- 🧵 `std::thread` for concurrency

---

## 📚 What I Learned

- Writing a protocol parser (RESP)
- Building a concurrent TCP server in Rust
- Managing safe shared state with threads
- Expiry strategies (inspired by Redis internals)

---

## 🧪 Testing

You can connect to your Rudis server using:

```bash
redis-cli -p 6379
```

Or by writing your own test clients using RESP.