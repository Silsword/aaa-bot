# AAA-bot
This is a university project. This bot is a telegram-based task-managment system 
with possibillity of setting due dates, changing states and other.

Bot writen in Rust with Teloxide framework.
## Building
Add your token to this line in main.rs:
```rust
	let bot = Bot::new("<token here>").auto_send();
```

```
$ git clone https://github.com/Silsword/aaa-bot
$ cd aaa-bot
$ cargo build --release
```
## Usage
Type `/start` to display welcome message.

