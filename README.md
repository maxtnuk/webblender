## Webblender

library for webgl with blender
( still developing it cannot use )

## To Do

- [ ] construct App
- [ ] combind bledner file

## How To use

first of all you need cargo-make

    $ cargo install cargo-make

and also need [NPM](https://www.npmjs.com/get-npm) for running web_client

to make client app and run server then run this

    $ cargo make init_server

or run this for build under root folder

    $ cargo make build

if you want to run server with npm, run this

    $ cargo make server

to wasm build run this

    $ cargo make wasm
