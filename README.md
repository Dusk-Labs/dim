# Dark Powered media Manager written in rust

## Current Design
![Design 1](./docs/design3.jpg?raw=true)

## Specification
Dim is a media manager powered by the dark forces. It is able to automatically scan the filesystem for movies, tv shows and parse game libraries. These items are then automatically added to their specific libraries which can then be accessed through a native or a comfy web ui.
To achieve this the application is split up into three main parts. It implements a micro-service type architecture. We have the front-end web-ui which is based on React.js, the backend server written in Rust utilizing the rocket web framework, diesel for the ORM and sqlite as the database, and lastly scanner scripts, these are scripts/programs which are called on server boot which scan the filesystem, parse metadata and fetch metadata for the media. So far only tv show and movie scanners are planned for implementation, with a steam library scanner coming later on.\

The scanners are split into two modules or sections. The first is a daemon that reacts on changes to the filesystem for example scanner 1 will monitor the changes to dir1, when a new file is created in dir1 the daemon calls a procedure to scan that and identify the file. The second part is the metadata scanner. The job of the metadata scanner is to scan the file and reduce the metadata for it. For example one could parse the file name or check for the metadata in the file itself then it should append it to the database through the use of various REST api endpoints. It can also update the data in the database. For example if the file gets moved it can automatically update the pointer in the database. Furthermore the scanners can fetch info from the net too. The scanners must take in at least 3 arguments, --endpoint, --auth-token --path. Each correspond to one attribute, first is for the ip/domain of the endpoint, second is the auth-token used to authenticate the scanner and the last is the path it will keep track of.

## Features
- Scan and automatically add, filter and fix media on your device
- Allow you to stream it over the network with no set up
- Be able to transcode if theres a need for example if the device doesnt support the codec
- Present a clean UI to see the media
- Allow you to remotely control the streaming session, for example to seek, pause, play, increase/decrease volume, or play something else
- Drag and Drop media upload/scan ( ie youre in the UI, you can drag and drop a file from your hard disk which will either get uploaded or if its from the same PC inserted into the library it was dragged to)
