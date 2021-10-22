<h1 align="center">Dim</h1>

![Dashboard](docs/design/dashboard.jpg)
[![Discord](https://img.shields.io/discord/834495310332035123)](http://discord.gg/YJCrFTykQ4)

Dim is a self-hosted media manager. With minimal setup, Dim will organize and beautify your media collections, letting you access and play them anytime from anywhere.

## Running from binaries
### Dependencies
  * libva2
  * libva-drm2
  * libharfbuzz
  * libfontconfig
  * libfribidi
  * libtheora
  * libvorbis
  * libvorbisenc

  You can then obtain binaries from the release tab in github:
  1. Unpack with `unzip ./release-linux.zip && tar -xvzf ./release.tar.gz`
  2. Run `cd release && ./dim`
  3. Then you can access the Dim web UI through your browser with `http://0.0.0.0:8000` (assuming it's running locally.)

## Running with docker
  * `docker run -d -p 8000:8000/tcp -v $HOME/.config/dim:/opt/dim/config -v /media:/media vgarleanu/dim:latest` 

### With hardware acceleration
  * `docker run -d -p 8000:8000/tcp -v $HOME/.config/dim:/opt/dim/config -v /media:/media --device=/dev/dri/renderD128 vgarleanu/dim:latest`

## Running from source
### Dependencies
  To run from source, you'll first need to install the following dependencies on your system:
  * sqlite
  * cargo
  * rustc (nightly)
  * yarn, npm
  * libssl-dev
  * libva2
  * libva-dev
  * libva-drm2
  * ffmpeg

  You can then clone the repository and build dim with the following commands:
  1. `git clone https://github.com/Dusk-Labs/dim`
  2. `yarn --cwd ui/ && yarn --cwd ui/ build`
  3. `cargo run --release`

## License
Dim is licensed under the GPLv2 license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/GPL-2.0)

## Screenshots
![Login_Page](docs/design/login_page.png)
![Add_Library Modal](docs/design/add_library.png)
![Media_Page](docs/design/media_page.jpg)
