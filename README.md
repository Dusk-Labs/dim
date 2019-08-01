# Dark Powered media Manager written in rust

## Current Design
![Design 1](./docs/design1.jpg?raw=true)

## Specification
Dim (temporary name) is a media manager. It is able to automatically scan the filesystem for movies, tv shows and parse game libraries. These items are then automatically added to their specific libraries which can then be accessed through a native or a comfy web ui.
To achieve this the application is split up into three main parts. It implements a micro-service type architecture. We have the front-end web-ui which is based on React.js, the backend server written in Rust utilizing the rocket web framework, diesel for the ORM and sqlite as the database, and lastly scanner scripts, these are scripts/programs which are called on server boot which scan the filesystem, parse metadata and fetch metadata for the media. So far only tv show and movie scanners are planned for implementation, with a steam library scanner coming later on.\

The scanners are split into two modules or sections. The first is a daemon that reacts on changes to the filesystem for example scanner 1 will monitor the changes to dir1, when a new file is created in dir1 the daemon calls a procedure to scan that and identify the file. The second part is the metadata scanner. The job of the metadata scanner is to scan the file and reduce the metadata for it. For example one could parse the file name or check for the metadata in the file itself then it should append it to the database through the use of various REST api endpoints. It can also update the data in the database. For example if the file gets moved it can automatically update the pointer in the database. Furthermore the scanners can fetch info from the net too. The scanners must take in at least 3 arguments, --endpoint, --auth-token --path. Each correspond to one attribute, first is for the ip/domain of the endpoint, second is the auth-token used to authenticate the scanner and the last is the path it will keep track of.

## Features
- Scan and automatically add, filter and fix media on your device
- Allow you to stream it over the network with no set up
- Be able to transcode if theres a need for example if the device doesnt support the codec
- Present a clean UI to see the media
- Allow you to remotely control the streaming session, for example to seek, pause, play, increase/decrease volume, or play something else


## API
### Authentication
- [ ] POST /api/v1/auth/session
    Data: username/password
    Returns: {JWT, Expiry}, 201
    ErrReturns: 401

- [ ] POST /api/v1/auth/refresh
    Use: Refresh the expiration of the token by sending it to this endpoint
    Data: JWT
    Returns: {JWT, Expiry}
    ErrReturns: 404/405 ?

- [ ] Delete /api/v1/auth/session
    Data: JWT,
    Returns: 201,
    ErrReturns: 404

### User/Accounts
- [ ] POST /api/v1/user/register
    Data: {username, password, email, recaptcha2}
    Returns: Redirect<Setup>, 200
    ErrReturns: 200 ErrMsg

- [ ] POST /api/v1/user/setup
    Data: {JWT, name, gender, avatar}
    Returns: 200 OK
    ErrReturns: 404

- [ ] DELETE /api/v1/user/
    Data: {username, password}

- [ ] PATCH /api/v1/user/
    Use: this request allows to modify password or critical info etc
    Data: {JWT, password, new_password} | {JWT, new_email}
    Returns: 200 OK
    ErrReturns: 403

### Library management
- [x] GET /api/v1/library
    Use: Get libraries in the database
    Data: {JWT}
    Returns: {id, name, path}

- [x] POST /api/v1/library
    Use: Add new library, type identifies which scanner it will use
    Data: {JWT, name, path, type}
    Returns: 201
    ErrReturns: 403

- [x] GET /api/v1/library/<id>
    Use: get info about the library, ie type, name, id
    Data: {JWT}
    Returns: 200 {id, name, type, ...}
    ErrReturns: 403

- [x] DELETE /api/v1/library/<id>
    Use: delete library and its subsequent children and data
    Data: {JWT}
    Returns: 201
    ErrReturns 404/403

- [ ] PATCH /api/v1/library/<id> ????
    Use: Modify data about the library, ie path, name
    Data: {JWT, new_data}
    Returns: 201
    ErrReturns: 404/403

- [ ] POST /api/v1/library/<id>/run
    Use: Run a action under the library which is usually handled by its scanners
    Data: {JWT, flag} Flag will usually be either scan or duplicatescan or fetch_metadata
    Returns: 201
    ErrReturns: 404/403 404 can be returned if the flag is not recognized

### Media management
- [x] POST /api/v1/library/<library_id>/media
    Use: Append a media object to the database
    Data: {JWT, item: {...}}
    Returns: 201, {id}
    ErrReturns: 403/404/500

- [ ] GET /api/v1/library/<library_id>/media<?sort=key&ascending=(true|false)>
    Use: Returns a list of ids of the media it holds, this can be sorted by whatever attributes the media has ascending or descending
    Data: {JWT}
    Returns: [ids...]
    ErrReturns: 404/403


### Media agnostic endpoints
- [x] GET /api/v1/media/<id>
    Use: Returns data about the media object, currently it is agnostic with tv shows because they share similar info
    Returns: 200, {data}
    ErrReturns: 404/403

- [ ] PATCH /api/v1/media/<id>
    Use: Patch info about the media object which is agnostic accross tv shows and movies
    Data: {new info}
    Returns: 201??
    ErrReturns: 404/403

- [ ] DELETE /api/v1/media/<id>
    Use delete the media object
    Returns: 201,
    ErrReturns 404/403

### Streamble media only
- [ ] POST /api/v1/stream-session/
    Use: Creates a new streaming session, the server automatically decides whether it will be a transcoding session or direct play
    Data: {media_id, video_offset, supported_codecs}
    Returns: 200, {stream_key}

- [ ] DELETE /api/v1/stream-session/<stream_key>
    Use: Stop the stream, if its a transcoding stream, it stops the transcoding process, otherwise its placebo
    Note: If the media is being transcoded, the chunks will get cached
    Returns: 201

- [ ] GET /api/v1/stream-session/<stream_key>
    Use: Get the media stream, returns whatever video data including useful headers
    Returns: 200 bytes(video_data)
    ErrReturns: 404

- [ ] GET /api/v1/user/stopped-at/<media_id>
    Use: Returns where the user has stopped at
    Returns: 200, {secs_offset}
    ErrReturns: 403/{0}

- [ ] POST /api/v1/user/stopped-at/<media_id>
    Use: log where the current media is at in secs
    Data: {secs_offset}
    Returns: 201
    ErrReturns: 403/404 404 if the media is not streamble or not found

### TV Show only
- [ ] GET /api/v1/tv/<media_id>
    Use: Mirrors /api/v1/media/<id>

- [ ] GET /api/v1/tv/<media_id>/season
    Use: Returns ids of its seasons
    Returns: [...ids]
    ErrReturns: 403/404

- [ ] GET /api/v1/season/<id>
    Use: Returns info about the season
    Returns {...}
    ErrReturns: 403/404

- [ ] POST /api/v1/tv/<media_id>/season
    Use: Add new season to the tv show with info
    Returns: {id}, 200
    ErrReturns: 403/404

- [ ] PATCH /api/v1/season/<id>
    Use: Patch info about the season
    Returns: 201,
    ErrReturns: 403/404

- [ ] DELETE /api/v1/season/<id>
    Use: Deletes the season
    Returns: 201

- [ ] POST /api/v1/season/<id>/episode
    Use: Push new episode to the season with info
    Data: {minimum episode info}
    Returns: 200, {id}

- [ ] GET /api/v1/episode/<id>
    Use: Get episode info
    Returns: 200 {info}

- [ ] PATCH /api/v1/episode/<id>
    Use: edit info about the episode
    Returns: 201

- [ ] DELETE /api/v1/episode/<id>
    Use: Deletes a episode
    Returns: 201
    ErrReturns: 403/404
