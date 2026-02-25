# Finicky Whiskers

Finicky Whiskers is a browser-based game in which the player attempts to feed their fickle pets. Built with [Spin](https://github.com/spinframework/spin) and WebAssembly, it demonstrates a microservices architecture running at the edge on Akamai.

## Architecture

Finicky Whiskers is comprised of several Spin components, all written in Rust:

- **fileserver** - Serves the static HTML, CSS, and JS assets
- **redirect** - Redirects `/` to `/index.html`
- **session** - Initializes game session data
- **tally** - Tallies individual game events
- **scoreboard** - Retrieves the current score
- **highscore** - Manages the high score leaderboard
- **reset** - Resets game state

## Prerequisites

- [Spin CLI](https://github.com/spinframework/spin) (latest)
- [Rust](https://rustup.rs/) with the `wasm32-wasip1` target
- [Node.js and npm](https://nodejs.org/)

Install the Rust Wasm target:

```console
rustup target add wasm32-wasip1
```

## Build

```console
spin build
```

## Run

```console
spin up
```

The game will be available at [http://127.0.0.1:3000](http://127.0.0.1:3000).

## Development

For working on the game UI (styles, etc):

```console
cd site
npm i
npm run styles
```

To run just the UI locally (without backend services), use [Parcel](https://parceljs.org/features/development/):

```console
cd site
npm run dev
```

Then view the site at [localhost:1234](http://localhost:1234/).
