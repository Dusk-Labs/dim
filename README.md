<h1 align="center">Dim</h1>

![Dashboard](./docs/design/dashboard.png?raw=true)

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


## License
Dim is licensed under the GPLv2 license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/GPL-2.0)

## [Gallery](docs/design/GALLERY.md)
