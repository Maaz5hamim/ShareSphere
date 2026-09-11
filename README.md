# ShareSphere

> **Share. Collaborate. Thrive.**

A full-stack peer-to-peer file sharing desktop application built from scratch in Rust, featuring a native GUI client and a concurrent async server with a MySQL backend.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-async-blue?style=for-the-badge)
![egui](https://img.shields.io/badge/egui-native--GUI-orange?style=for-the-badge)
![MySQL](https://img.shields.io/badge/MySQL-4479A1?style=for-the-badge&logo=mysql&logoColor=white)

---

## What It Does

ShareSphere lets users upload files to a central server and selectively share them with other registered users — without any cloud service or third-party dependency. Everything runs over raw TCP, serialized with binary encoding for maximum efficiency.

- Upload a file → it lands on the server, indexed to your account
- Share it → specify any registered emails and they instantly see it in their "Shared With Me" view
- Download or delete files from anywhere on the network
- Browse everything through a polished native desktop interface

---

## Architecture

```
┌─────────────────────────────────┐        TCP (bincode)        ┌──────────────────────────────────┐
│         CLIENT (egui)           │ ◄─────────────────────────► │          SERVER (Tokio)          │
│                                 │                             │                                  │
│  ┌──────────┐  ┌─────────────┐  │     Request { email,        │  ┌────────────┐  ┌─────────────  │
│  │  UI      │  │  request/   │  │       service: RequestType} │  │  request_  │  │  service/   │ │
│  │  (pages) │─►│  service.rs │──┼────────────────────────────►│  │  handler   │─►│  database   │ │
│  └──────────┘  └─────────────┘  │                             │  └────────────┘  └─────────────┘ │
│                                 │◄────────────────────────────┤                      │           │
│  Pages: Splash, Login, Signup,  │      Response enum          │                  MySQL DB        │
│         Home, MyFiles,          │                             │ (user, file, shared_with tables) │
│         SharedFiles, Folder     │                             │                                  │
└─────────────────────────────────┘                             └──────────────────────────────────┘
```

---

## Features

| Feature | Details |
|---|---|
| **User Auth** | Register and log in with email + password, verified against MySQL |
| **File Upload** | Stream any file over TCP in 10 KB chunks to the server |
| **File Download** | Download your own uploads or files shared with you to any local path |
| **File Sharing** | Share any of your uploads with multiple users by email in one request |
| **My Uploads** | Dashboard showing all your files with size, date, and who you shared with |
| **Shared With Me** | Browse files shared by others, organized into per-user folders |
| **Delete** | Remove a file from the server and database atomically |
| **Dark / Light Theme** | One-click theme toggle with full egui visual overhaul |
| **Splash Screen** | Branded splash screen on startup |

---

## Tech Stack

### Client — `client/`

| Crate | Role |
|---|---|
| [`eframe` / `egui`](https://github.com/emilk/egui) | Immediate-mode native GUI framework — no HTML, no Electron, pure Rust |
| [`tokio`](https://tokio.rs) | Async runtime for non-blocking TCP calls from the GUI thread |
| [`serde`](https://serde.rs) + [`bincode`](https://github.com/bincode-org/bincode) | Zero-copy binary serialization of the request/response protocol |
| [`rfd`](https://github.com/PolyMeilex/rfd) | Native OS file/folder picker dialogs |
| [`egui_extras`](https://docs.rs/egui_extras) | Image rendering (splash, login/signup backgrounds) |

### Server — `server/`

| Crate | Role |
|---|---|
| [`tokio`](https://tokio.rs) | Async TCP listener; each client connection spawned as an independent task |
| [`sqlx`](https://github.com/launchbadge/sqlx) | Compile-time checked async MySQL queries |
| [`serde`](https://serde.rs) + [`bincode`](https://github.com/bincode-org/bincode) | Shared binary protocol with the client |

---

## Protocol Design

The client and server speak a custom binary protocol over raw TCP:

```rust
// Sent from client → server
struct Request {
    email: String,
    service: RequestType,
}

enum RequestType {
    Login { password },
    Signup { name, password },
    UploadFile { file_path },
    DownloadFile { file_name, file_path },
    DeleteFile { file_name },
    MyUploadList,
    GetFileNames { folder_name },
    GetFolderNames,
    Share { members: Vec<String>, filename },
}

// Sent from server → client
enum Response {
    Success,
    Failure(String),
    Initiate,                                   // server ready to receive/send file bytes
    GetFileNameSuccess { filenames: Vec<File> },
    GetFolderNameSuccess { foldernames: Vec<Folder> },
    MyUploadListSuccess { list: Vec<MyFile> },
}
```

File transfers use a two-phase handshake: the server sends `Response::Initiate` to signal readiness, then raw bytes stream over the same connection.

---

## Database Schema

```sql
CREATE TABLE user (
    name     VARCHAR(100),
    email    VARCHAR(100) PRIMARY KEY,
    password VARCHAR(100)
);

CREATE TABLE file (
    file_id     INT AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(255),
    path        VARCHAR(500),
    owner_id    VARCHAR(100) REFERENCES user(email),
    size        INT UNSIGNED,
    upload_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE shared_with (
    file_id INT REFERENCES file(file_id),
    email   VARCHAR(100) REFERENCES user(email),
    PRIMARY KEY (file_id, email)
);
```

---

## Project Structure

```
ShareSphere/
├── Cargo.toml               # Workspace manifest
│
├── client/                  # Desktop GUI application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # eframe entry point
│       ├── UI.rs            # All GUI pages and state machine
│       ├── request.rs       # Protocol types + request dispatcher
│       └── request/
│           └── service.rs   # Async TCP service calls (one per RequestType)
│
└── server/                  # Concurrent TCP file server
    ├── Cargo.toml
    └── src/
        ├── main.rs                        # TcpListener + task spawner
        ├── request_handler.rs             # Deserialize request, dispatch handler, send response
        └── request_handler/
            ├── service.rs                 # Business logic (auth, file I/O)
            └── service/
                └── database.rs           # All sqlx MySQL queries
```

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs) (stable)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- MySQL server (5.7+ or MariaDB 10.3+)

  On macOS: `brew install mysql && brew services start mysql`

### 1 — Set up the database

```bash
mysql -u root -p < schema.sql
```

That creates the `sharesphere` database and all three tables in one step.

### 2 — Configure the server

The server ships with sensible defaults but you need to point it at your MySQL instance. Create `server/Config.toml`:

```toml
[network]
server_addr = "0.0.0.0:8080"

[database]
url = "mysql://root:yourpassword@localhost/sharesphere"

[storage]
path = "/path/to/storage"
```

Place `Config.toml` next to the server binary (or in the directory you run it from). If no file is found, the server starts with its built-in defaults.

### 3 — Configure the client

The client also works out of the box (it launches in demo mode). To connect to a real server, create `client/Config.toml`:

```toml
[network]
server_addr = "192.168.1.100:8080"   # your server's LAN IP

[mock]
enabled = false
```

Place `Config.toml` next to the client binary (or in the directory you run it from).

### 4 — Start the server

```bash
cd server
cargo run
```

You should see: `Server listening on <addr>`

### 5 — Start the client

Open a second terminal:

```bash
cd client
cargo run
```

The ShareSphere window opens with a splash screen, then drops you into the login page.

### Build release binaries

```bash
cd server && cargo build --release
cd client && cargo build --release
# Binaries land at:
#   server/target/release/sharesphere-server
#   client/target/release/sharesphere-client
```

---

## Who Is It For

ShareSphere is built for **small teams and businesses that need to share files internally without depending on cloud storage services** like Google Drive or Dropbox.

If your team works on a local network — a studio, a small office, a lab, a startup — you can run a single ShareSphere server on any machine and every person on the network gets a full file-sharing platform: upload, organize, and share files directly with colleagues by email, all within your own infrastructure.

**Common use cases:**

- A design agency sharing large asset files between team members without hitting cloud storage limits or paying per-seat fees
- A small business keeping sensitive documents off third-party servers entirely
- A research team on an air-gapped or restricted network that can't use public cloud services
- Any team that wants to own their data and infrastructure without the complexity of self-hosting an enterprise solution

No subscription. No cloud dependency. No data leaving your network. Just run the server, point everyone at it, and start sharing.

---

## License

MIT
