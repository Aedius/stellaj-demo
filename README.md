# stellaj-demo

## install

to build the frontend it need trunk

`
cargo install trunk
`

## build

`
trunk build --dist stellaj-server/web stellaj-client/index.html

cargo build -p stellaj-server
`

## Todo : 

- cleanup stream on close
- cleanup stream on kill*
- handle all errors 😨