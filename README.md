<h1 align="center">Dim</h1>

![dashboard](https://user-images.githubusercontent.com/44278658/116753720-71724180-a9ff-11eb-8ac0-6fe4df85e63f.png)

Dim is a self hosted media manager. With minimal setup, Dim will organize and beautify your media collections, letting you access and play them anytime from any browser window.

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

## Screenshots
![Login_Page](https://user-images.githubusercontent.com/44278658/116753932-d168e800-a9ff-11eb-9714-40ea54ef78e6.png)
![Add_Library Modal](https://user-images.githubusercontent.com/44278658/116754109-14c35680-aa00-11eb-96d2-eb692d57f1da.png)
![Media_Page](https://user-images.githubusercontent.com/44278658/116754147-24429f80-aa00-11eb-9416-e1ab60f3f1ea.png)
