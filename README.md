# re203 — Peer-to-Peer File Sharing System

A P2P file sharing application implementing a BitTorrent-like protocol with a centralized tracker. The system is composed of two independent components:

| Component | Language | Description |
|-----------|----------|-------------|
| **tracker/** | C | Centralized tracker server that indexes files and routes peers |
| **peer/** | Rust + JS/HTML (Tauri) | Desktop peer client with a GUI frontend |

## Architecture

```
          ┌──────────┐
          │ Tracker  │
          │  (C)     │
          └────┬─────┘
               │
    ┌──────────┼──────────┐
    │          │          │
    ▼          ▼          ▼
┌────────┐ ┌────────┐ ┌────────┐
│ Peer A │◄►│ Peer B │◄►│ Peer C │
│(Tauri) │  │(Tauri) │  │(Tauri) │
└────────┘ └────────┘ └────────┘
```

All peers maintain a persistent connection to the tracker. They register themselves and their shared files with the tracker, then query it to discover which peers have which files. Once a peer knows who has the desired file, it connects **directly** to those peers to exchange file pieces — all peer-to-peer data transfer happens without going through the tracker.

### Protocol Messages

The peers and tracker communicate over TCP using a text-based protocol:

| Message | Direction | Description |
|---------|-----------|-------------|
| `announce listen <port> seed [<files>] leech [<hashes>]` | Peer → Tracker | Register files this peer is seeding/leeching |
| `look [<criteria>]` | Peer → Tracker | Search for files by name and/or size |
| `getfile <hash>` | Peer → Tracker | Request the list of peers sharing a specific file |
| `update seed [<hashes>] leech [<hashes>]` | Peer → Tracker | Refresh this peer's registration (keep-alive) |
| `interested <hash>` | Peer → Peer | Signal interest in a file |
| `have <hash> <buffermap>` | Peer → Peer | Announce which pieces of a file a peer possesses |
| `getpieces <hash> [<indexes>]` | Peer → Peer | Request specific pieces of a file |
| `data <hash> [<piece-data>]` | Peer → Peer | Send requested file pieces |

Files are identified by their MD5 hash and exchanged in fixed-size pieces (chunks). The tracker periodically evicts entries that have not sent an `update` within the configured time-to-live.

---

## Project Structure

```
re203/
├── tracker/                  # C tracker server
│   ├── src/
│   │   ├── main.c            # Entry point, accept loop
│   │   ├── tracker.c/h       # Core tracker logic, socket creation
│   │   ├── network.c/h       # IP detection, public IP fetch
│   │   ├── parser.c/h        # Protocol message parsing (regex-based)
│   │   ├── database.c/h      # In-memory file/peer index
│   │   ├── threads.c/h       # Thread pool for concurrent clients
│   │   ├── logging.c/h       # Colored log levels, file logging
│   │   ├── args.c/h          # CLI argument parsing (-p, -c, -v, -m)
│   │   └── parameters.c/h   # Config file loading (INI format)
│   ├── tst/                  # Unit tests
│   ├── config.ini            # Default tracker configuration
│   ├── Makefile
│   └── mock_peer.py          # Python mock peer for testing
│
├── peer/                     # Tauri desktop application (Rust backend + JS frontend)
│   ├── src/
│   │   ├── index.html        # Main GUI page
│   │   ├── main.js           # Frontend logic (search, download, dashboard)
│   │   └── styles.css        # Styling
│   ├── src-tauri/
│   │   ├── src/
│   │   │   ├── main.rs       # Tauri app setup, CLI args, interface selection
│   │   │   ├── back.rs       # Download orchestration, piece management
│   │   │   ├── com.rs        # Protocol message formatting, TCP send/receive
│   │   │   ├── data.rs       # Shared data types and global configuration
│   │   │   ├── db.rs         # Local file/piece tracking (seeding & leeching state)
│   │   │   ├── for_frontend.rs  # Tauri commands exposed to the JS frontend
│   │   │   ├── parser.rs     # Response parsing from tracker/peers
│   │   │   ├── process.rs    # File processing, hashing, splitting
│   │   │   ├── threads.rs    # Thread pool implementation
│   │   │   └── ...           # Additional modules
│   │   ├── Cargo.toml        # Rust dependencies
│   │   ├── tauri.conf.json   # Tauri window/bundle config
│   │   ├── build.rs
│   │   └── config.ini        # Default peer configuration
│   ├── Makefile
│   └── package.json
│
└── README.md                 # You are here
```

---

## Prerequisites

### Tracker (C)

- **GCC** — the C compiler
- **POSIX threads** (`pthread`) — usually available by default on Linux
- **cURL** — for public IP detection (`apt install curl`)

### Peer (Tauri / Rust)

- **Rust toolchain** — install via [rustup](https://rustup.rs/)
- **Node.js & npm** — for the Tauri CLI frontend tooling
- **Tauri system dependencies** (Linux/Debian):

  ```bash
  sudo apt install \
    libsoup2.4-dev \
    libpango1.0-dev \
    libgdk-pixbuf-2.0-dev \
    libgtk-3-dev \
    libjavascriptcoregtk-4.0-dev \
    libwebkit2gtk-4.0-dev
  ```

  > See the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for other platforms.

- **npm dependencies:**

  ```bash
  cd peer/
  npm install
  ```

---

## Building & Running

### Tracker

```bash
cd tracker/

# Debug build
make

# Run with default config
make run
# Equivalent to: ./tracker -v 2 -m 1

# Release build (optimized)
make release

# Run unit tests
make test
make check

# Clean build artifacts
make clean
```

**CLI options:**

```
-p, --port <port>        Listen port (overrides config.ini)
-c, --config <path>      Path to config file (default: ./config.ini)
-v, --verbose <level>    Log verbosity: 0=NONE, 1=ERROR, 2=WARNING (default), 3=LOG, 4=DEBUG
-m, --max-conn <n>       Maximum concurrent connections (thread pool size)
```

**Configuration** (`config.ini`):

```ini
[network]
port=12345
max-conn=5
cache-time=300
```

- `port` — TCP port the tracker listens on
- `max-conn` — thread pool size for handling concurrent peer connections
- `cache-time` — time-to-live (seconds) before inactive peer entries are evicted

### Peer

```bash
cd peer/

# Install npm dependencies (first time only)
npm install

# Dev mode (GUI with hot-reload)
make run
# This also copies the binary and config to build/

# Release build (optimized standalone binary)
make run-release
# Builds and launches the release binary

# Terminal-only mode (no GUI, CLI interface)
make run-terminal

# Build without running
make

# Run Rust unit tests
make test

# Clean
make clean
```

**CLI options:**

```
-c, --config <path>      Path to config file (default: ./config.ini)
-i, --interface <mode>   Interface mode: "tauri" (GUI) or "terminal" (CLI)
```

**Configuration** (`src-tauri/config.ini`):

```ini
[Tracker]
tracker-address = jibelibeju.fr
tracker-port = 12345

[Peer]
peer-address = 0.0.0.0
peer-port = 54321
log-level = "trace"
max-connections = 1
length-tcp = 51200
update-period = 30
interface = "tauri"
```

- `tracker-address` / `tracker-port` — the tracker to connect to
- `peer-port` — TCP port this peer listens on for other peers
- `log-level` — Rust log level (`trace`, `debug`, `info`, `warn`, `error`)
- `max-connections` — max concurrent peer connections
- `length-tcp` — TCP buffer/chunk size in bytes
- `update-period` — seconds between tracker keep-alive updates
- `interface` — default interface mode (`"tauri"` or `"terminal"`)

---

## Quick Start

1. **Start the tracker** in one terminal:

   ```bash
   cd tracker/
   make run
   ```

2. **Start a peer** in another terminal:

   ```bash
   cd peer/
   npm install   # first time only
   make run
   ```

3. In the GUI:
   - Click **"Upload Files"** to select files to seed.
   - Click **"Search File to Download"** to look for files on the network by name/size.
   - Click on a search result row to start downloading from available peers.
   - The **dashboard** auto-refreshes to show download progress and connected peers.

4. To test the tracker without a full peer, use the mock peer script:

   ```bash
   cd tracker/
   python3 src/mock_peer.py
   ```

---

## Testing

### Tracker

```bash
cd tracker/
make test       # compile and run unit tests
make check      # compile tests, run, and verify output
```

Tests cover the database, parser, network, threading, logging, and tracker modules.

### Peer

```bash
cd peer/
make test       # run Rust unit tests (single-threaded)
```

---

## Build Targets Summary

| `make` target | Component | Description |
|---------------|-----------|-------------|
| `all` (default) | tracker | Build debug binary |
| `run` | tracker | Build & run with default options |
| `release` | tracker | Build optimized binary (`-O3`) |
| `test` | tracker | Compile & run unit tests |
| `check` | tracker | Run tests and verify |
| `clean` | tracker | Remove build artifacts |
| `all` (default) | peer | Build Tauri app (debug) via Cargo |
| `run` | peer | Launch Tauri dev mode (GUI with hot-reload) |
| `run-release` | peer | Build release binary & launch |
| `run-terminal` | peer | Build release binary & run in terminal mode |
| `test` | peer | Run Cargo unit tests |
| `clean` | peer | `cargo clean` + remove build/ |

---

## License

This project was developed as part of the RE203 course.
