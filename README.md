# sendtelegram - send messages to a private Telegram chat

sendteltgream is a tiny tool, to send messages to a private Telegram channel.
It accepts the message to send via a comandline argument or pipe. A UTF icon can be added.

## Compilation
```bash
~# git clone XXX
~# cd sendtelegram
~/sendtelegram# cargo build --release
```
The resuling binary is `./target/release/sendtelegram.`

## Installation
You need to create a private Telegram chat. This will provide you with an API key and a CHAT ID.
Both must be entered into `/etc/sendTelegram.cfg`. There is an example configuration file: `./sentelegram/res/sendTelegram.cfg`

## Usage
```bash
~/sendtelegram# ./target/release/sendtelegram -m "Hello" -C Bell
~/sendtelegram# echo "World" | ./target/release/sendtelegram
```

NOte: This is my very first Rust code.
