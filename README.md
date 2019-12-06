# Dark Powered media Manager written in rust
![Design 1](./docs/design/image6.jpg?raw=true)

## What is Dim?

Dim is a self hosted media manager powered by the dark forces. It is able to automatically scan the filesystem for movies and tv shows. These items are then automatically added to their specific libraries which can then be accessed through a comfy web ui. It is an open source alternative to Plex, with straightforward media management features and no bloat.

## Why another media manager?
This project was started after seeing how other media managers like Plex and Emby are taking a closed source route, diverting from their original mission. Plex is too centralized for my taste, Dim is fully self hosted and open source.

## Tech stack
Dim is mainly written in Rust and JS. We use Rocket as our webserver paired with Diesel as the ORM. For the Web UI we use React.js. The current database system is PostgreSQL. Dim is currently in Alpha testing with some features not complete yet.

## Features
### Server
- [x] Movie and TV Show scanners
- [x] Media matcher w/ resource fetch
- [x] Library APIs(add new, delete, rename)
- [x] Media APIs
- [x] Streaming APIs(start, stop)
- [x] Transmuxing (only into h264 hls)
- [ ] Transcoding
- [x] Event APIs(new_library, delete_library, new_media, delete_media)
- [x] Title Search
- [ ] Auth (registration not done, login done)
- [x] Advanced Search (by genre, year, title, or all three at once)
- [x] Offline mode post scan
- [ ] Offline mode prescan
- [ ] Chromecast support
- [ ] Mixed content type library support (properly scan and display)
- [ ] Anime scanners
- [ ] Server stats for the dashboard etc..
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
- [ ] Server stats in dashboard
### General
- [x] Server support for Linux
- [ ] Server support for WSL (partial, database setup is a bit hacky)
- [ ] Server support for Windows (needs testing)
- [ ] Server support for BSD (needs testing)
- [ ] Better docs and more unit tests (coverage is quite small atm, only covering a good half of the database module)
- [ ] More streamlined build process
- [ ] Precompiled binaries
### Longterm
- [ ] Phone apps (Android, ios)
- [ ] Roku, nvidia shield, firestick apps

## Installation
### Dependencies
1. libpg, libpg-dev
2. ffmpeg, ffprobe
3. rustc nightly >= 1.40.0

1. First set up the database with docker: `docker-compose up -d postgres`
2. Next build the UI: `cd web_ui && yarn build`
3. Lastly run Dim: `cargo run --release`

Dim runs by default on port 8000.

## Contributing
Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Contribute code via [merge requests].

[issue]: https://gitlab.com/vgarleanu/dim/issues
[merge requests]: https://gitlab.com/vgarleanu/dim/merge_requests

All pull requests are code reviewed and tested by the CI. Note that unless you
explicitly state otherwise, any contribution intentionally submitted for
inclusion in PushEvent by you shall be licensed under the GNU GPLv2 License 
without any additional terms or conditions.

## License
Dim is licensed under the GPLv2 license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/GPL-2.0)

## [Gallery](docs/design/GALLERY.md)
