<h1 align="center">Dim</h1>
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
