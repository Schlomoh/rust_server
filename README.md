# Rust Server
## Intro

### This rust server shall act as my interface to access different cryptocurrency data

As my first *"real world"* rust project, I am creating this rust server, as a way to serve current cryptocurrency data, then displayed in a next.js front-end.
All prices shall be gathered from different exchanges to either be averaged or made available on their own. These prices will also be interpreted by my indicators to then produce bullish/bearish signals. 🤞🏼

This could then be furthermore extended to create trades implementing those signals.

The main goal is to use the speed of rust doing all kinds of calculations in the back-end having the data all ready to be displayed in the front-end for monitoring purposes.

#### Ideas

I've thought about using the rarity.tools site to look for NFTs offered for a price that's not corresponding to their rank on the site. 
So an NFT that's sold for 200 bucks while ones on a similar rank are sold for 500 would be a good opportunity. This asset could then be bought and afterwards sold for a more appropriate price. 

I'll have to look for a way to access the data on their site first though. 👀


## Usage
#### Starting the server without logging
```
>> cargo run

or 

>> cargo build --release
>> ./target/release/rust_server 
```

#### Starting with logging
```

>> cargo run info

or 

>> cargo build --release
>> ./target/release/rust_server info
```
