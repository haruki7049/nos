# nos

A Nostr CLI client which can post text note to nostr network only

```sh
$ nos -m "Hello, Nostr!!"
Sent your message to Nostr relays: "Hello, Nostr!!"
```

This program uses the rust crate [confy](https://github.com/rust-cli/confy). You must write your config file on `~/.config/nos/config.toml` for Linux, or on `~/Library/Application Support/rs.nos/config.toml` on macOS.

Configuration format is the following:

```toml
# relay list
relays = ["wss://yabu.me"]

# Your seckey
seckey = "nsecAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
```
