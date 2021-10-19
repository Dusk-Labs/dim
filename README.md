<h1 align="center">Dim</h1>

![Dashboard](https://user-images.githubusercontent.com/44278658/116753720-71724180-a9ff-11eb-8ac0-6fe4df85e63f.png)

Dim is a self hosted media manager. With minimal setup, Dim will organize and beautify your media collections, letting you access and play them anytime from anywhere.

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

## Running with docker
  1. `docker run -d -p 8000:8000/tcp --mount type=bind,source=$HOME/.config/dim,target=/opt/dim/config --mount type=bind,source=/media,target=/media vgarleanu/dim:latest` 

  You can also run dim with hardware accelerated transcoding enabled with:
  2. `docker run -d -p 8000:8000/tcp --mount type=bind,source=$HOME/.config/dim,target=/opt/dim/config --mount type=bind,source=/media,target=/media --device=/dev/dri/renderD128 vgarleanu/dim:latest`

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
![Login_Page](https://user-images.githubusercontent.com/44278658/116753932-d168e800-a9ff-11eb-9714-40ea54ef78e6.png)
![Add_Library Modal](https://user-images.githubusercontent.com/44278658/116754109-14c35680-aa00-11eb-96d2-eb692d57f1da.png)
![Media_Page](https://user-images.githubusercontent.com/44278658/116754147-24429f80-aa00-11eb-9416-e1ab60f3f1ea.png)
