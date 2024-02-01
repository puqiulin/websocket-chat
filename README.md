# Websocket chat server
> Based on [tokio_tungstenite](https://github.com/snapview/tokio-tungstenite/tree/master)

## Run server

```shell
cargo run
```

## Run client

### Install client tool [websocat](https://github.com/vi/websocat)

```shell
brew install websocat
```

### Run

```shell

websocat ws://127.0.0.1:9595
```
