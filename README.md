# Dark Powered media Manager written in rust

## Current Design
![Design 1](./docs/design3.jpg?raw=true)

## Specification
Dim is a media manager powered by the dark forces. It is able to automatically scan the filesystem for movies, tv shows and and other planned media types. These items are then automatically added to their specific libraries which can then be accessed through a native or a comfy web ui.
To achieve this the application is split up into two parts. We have the front-end web-ui which is based on React.js, the backend server written in Rust utilizing the rocket web framework, diesel for the ORM and postgres as the database.

## Features
- Scan and automatically add, filter and fix media on your device
- Allow you to stream it over the network with no set up
- Be able to transcode if theres a need for example if the device doesnt support the codec
- Present a clean UI to see the media
- Allow you to remotely control the streaming session, for example to seek, pause, play, increase/decrease volume, or play something else
- Drag and Drop media upload/scan ( ie youre in the UI, you can drag and drop a file from your hard disk which will either get uploaded or if its from the same PC inserted into the library it was dragged to)
