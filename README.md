# Dark Powered media Manager written in rust (Alpha)
![Design 1](./docs/design/image6.jpg?raw=true)

## Status
Currently Dim's development is paused since we can't get fucking streaming to work.

## What is Dim?

Dim is a self hosted media manager powered by the dark forces. It is able to automatically scan the filesystem for movies and tv shows. These items are then automatically added to their specific libraries which can then be accessed through a comfy web ui. It is an open source alternative to Plex, with straightforward media management features and no bloat.

## Why another media manager?
This project was started after seeing how other media managers like Plex and Emby are taking a closed source route, diverting from their original mission. Plex is too centralized for my taste, Dim is fully self hosted and open source and comes with a main focus on solid features then UX.

## Tech stack
Dim is mainly written in Rust and JS. We use Rocket as our webserver paired with Diesel as the ORM. For the Web UI we use React.js. The current database system is PostgreSQL. Dim is currently in Alpha testing with some features not complete yet.

## MVP Release at
- [x] TV Show/Movie scanners
- [x] Basic webui with dashboard, ability to add libraries etc...
- [ ] Video player with transmuxing/transcoding (server + ui)
- [x] 90% of unwraps removed from the server codebase

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
- [x] Auth
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
- [x] Live events (awaiting API integration)
- [ ] Extended Media pages
- [x] Authentication
- [ ] Server stats in dashboard
### General
- [x] Server support for Linux
- [ ] Server support for WSL (partial, database setup is a bit hacky)
- [ ] Server support for Windows (needs testing)
- [ ] Server support for BSD (needs testing)
- [ ] Better docs and more unit tests (coverage is quite small atm, only covering a good half of the database module)
- [x] More streamlined build process
- [ ] Precompiled binaries
- [x] Docker images
### Longterm
- [ ] Phone apps (Android, ios)
- [ ] Roku, nvidia shield, firestick apps

## Installation
  1. `docker volume create dim` \
  2. `docker run -d -p 8000:8000/tcp -p 3012:3012/tcp \` \
`        --mount source=dim,target="/var/lib/postgresql/" \` \
`        --mount type=bind,source="$HOME/media",target=/media \` \
`        vgarleanu/dim-server:latest`

Dim runs by on port 8000.

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
