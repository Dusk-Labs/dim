# Dark Powered media Manager written in rust (Alpha)
![Design 1](./docs/design/current.jpg?raw=true)

Dim is a self hosted media manager. 

## Installation (Docker)
  1. `docker volume create dim` \
  2. `docker run -d -p 8000:8000/tcp -p 3012:3012/tcp \` \
`        --mount source=dim,target="/var/lib/postgresql/" \` \
`        --mount type=bind,source="$HOME/media",target=/media \` \
`        vgarleanu/dim-server:latest`

## Installation (From source)
  1. `git clone git@github.com:vgarleanu/dim.git`
  2. `cargo run`

## Features
### Server
- [x] Movie and TV Show scanners
- [x] Media matcher w/ resource fetch
- [x] Library APIs
- [x] Media APIs
- [x] Streaming APIs
- [x] Transmuxing
- [x] Transcoding
- [x] Event APIs(new_library, delete_library, new_media, delete_media)
- [x] Title Search
- [x] Auth
- [x] Advanced Search (by genre, year, title, or all three at once)
- [ ] Mixed content type library support (properly scan and display)
- [ ] Anime scanners
- [ ] Server stats for the dashboard etc..
### Web UI
- [x] Dashboard w/ banners
- [x] Library views
- [x] New library modals
- [x] Card popouts
- [x] Search
- [x] Video player
- [x] Live events (awaiting API integration)
- [x] Extended Media pages
- [x] Authentication
- [ ] Server stats in dashboard
### General
- [x] Server support for Linux
- [x] Server support for WSL (SQLite support only)
- [x] Server support for Windows
- [ ] Server support for BSD
- [ ] Precompiled binaries
- [x] Docker images
### Longterm
- [ ] Phone apps (Android, ios)
- [ ] Roku, nvidia shield, firestick apps

## Contributing
Contributions are absolutely, positively welcome and encouraged! Contributions
come in many forms. You could:

  1. Submit a feature request or bug report as an [issue].
  2. Ask for improved documentation as an [issue].
  3. Contribute code via [pull requests].

[issue]: https://github.com/vgarleanu/dim/issues
[pull_requests]: https://github.com/vgarleanu/dim/pulls

All pull requests are code reviewed and tested by the CI. Note that unless you
explicitly state otherwise, any contribution intentionally submitted for
inclusion in PushEvent by you shall be licensed under the GNU GPLv2 License 
without any additional terms or conditions.

## License
Dim is licensed under the GPLv2 license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/GPL-2.0)

## [Gallery](docs/design/GALLERY.md)
