# scatterbrain-rs: rust wrapper for scatterbrain desktop API
This is a WIP wrapper for the desktop api for controlling a scatterbrain router
running on android phone from a desktop application. There is no formal
documentation for this protocol as of now and the protocol itself is under heavy
development.

## Architecture
Each app can discover nearby scatterbrain routers via mdns, or connect to
them directly. Once connected apps use a simple message oriented API
to perform the same functions that the
[android api](https://github.com/Scatterbrain-DTN/ScatterbrainSDK)
is capable of. This includes
- send and receive messages
- interact with identities
- receive network events

## Development goals
- integration with pure rust applications
- stable C/cdylib ABI
- integration wtih flutter apps
- potentially integration with nodejs/electron?
