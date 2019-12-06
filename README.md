# Dark Powered media Manager written in rust
![Design 1](./docs/design/design3.jpg?raw=true)

## What is Dim?

Dim is a media manager powered by the dark forces. It is able to automatically scan the filesystem for movies, tv shows and and other planned media types. These items are then automatically added to their specific libraries which can then be accessed through a native or a comfy web ui.

## Tech stack
Dim is mainly written in Rust and JS. We use Rocket as our webserver paired with Diesel as the ORM. For the Web UI we use React.js. The current database system is PostgreSQL. Dim is currently in Alpha testing with some features not complete yet.

## Features
### Server
- [x] Movie and TV Show scanners
- [x] Media matcher w/ resource fetch
- [x] Library APIs(add new, delete, rename)
- [x] Media APIs
- [x] Streaming APIs(start, stop)
- [x] Event APIs(new_library, delete_library, new_media, delete_media)
- [x] Title Search
- [ ] Auth (registration not done, login done)
- [ ] Advanced Search
### Web UI
- [x] Dashboard w/ banners
- [x] Library views
- [x] New library modals
- [x] Card popouts
- [x] Search
- [ ] Video player (partially done, awaiting API integration)
- [ ] Live events (awaiting API integration)
- [ ] Extended Media pages
- [ ] Authentication
### General
- [ ] More streamlined build process
- [ ] Precompiled binaries

## Installation
### Dependencies
1. libpg, libpg-dev
2. ffmpeg, ffprobe

First set up the database with docker: `docker-compose up -d postgres`
Next build the UI: `cd web_ui && yarn build`
Lastly run Dim: `cargo run --release`

Dim runs by default on port 8000.
