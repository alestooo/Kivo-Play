# Kivo Play

Open-source multiplayer mini-game platform for fast solo and party games, private rooms, real-time scoring, and extensible game modes.

## About

Kivo Play is a multiplayer mini-game platform focused on short, competitive, and social game sessions.

The first version will include private rooms, real-time gameplay, server-authoritative scoring, multiple rounds, leaderboards, and final podium results.

## Kivo Play v0.1

Initial games:

- Scale
- Estimate
- Higher / Lower

Core features:

- Guest players
- Private rooms
- Room codes
- Ready / Not Ready system
- Real-time multiplayer
- Multiple rounds
- Server-side scoring
- Scoreboard
- Final podium
- Play Again

## Tech Stack

### Frontend

- SvelteKit
- TypeScript

### Backend

- Rust
- Axum
- Tokio

### Real-time

- WebSockets

Future technologies may include PostgreSQL, Redis, Phaser or PixiJS, Godot, and Tauri.

## Architecture

Kivo Play follows a server-authoritative architecture.

Clients send player actions and answers, while the server validates game state, timing, scoring, permissions, and results.

```text
SvelteKit Client
       │
       │ HTTP / WebSocket
       ▼
Rust + Axum Server
       │
       ├── Rooms
       ├── Players
       ├── Game State
       ├── Rounds
       ├── Scoring
       └── Results
```

## Repository Structure

```text
kivo-play/
├── apps/
│   ├── web/
│   └── server/
├── docs/
├── .github/
├── README.md
├── LICENSE
└── .gitignore
```

The structure will grow only when new components are actually required.

## Roadmap

Future versions may include:

- Card Clash
- Sudoku Rush
- Quick Chess
- Mini Golf
- Party Mix
- Public rooms
- Matchmaking
- Player profiles
- Statistics
- Achievements
- PostgreSQL
- Redis
- Phaser / PixiJS
- Godot
- Tauri desktop app
- Kivo Play Game SDK

## Development Status

Kivo Play is currently in early development.

Target release:

**Kivo Play v0.1.0**

## License

MIT License