# Rust Server
## Intro

### This rust server shall act as my personal interface to access different cryptocurrency data

As my first *"real world"* rust project, I am creating this rust server, serving current cryptocurrency prices then displayed in a next.js front-end.
These prices shall be gathered from different exchanges to then be interpreted by my own indicators to finally produce clear bullish/bearish signals.

This could then furthermore be extended to create trades implementing those signals.

## Usage
#### starting the server without logging
```
>> cargo run

or 

>> cargo build --release
>> ./target/release/rust_server 
```

#### starting with logging
```

>> cargo run info

or 

>> cargo build --release
>> ./target/release/rust_server info
```
