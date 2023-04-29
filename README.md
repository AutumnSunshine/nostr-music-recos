# nostr-music-recos
Basic Rust program that extracts Spotify, Tidal &amp; Apple Music tracks recommended on Nostr on a given day &amp; publishes compiled list as notes on Nostr

Makes use of Rust crates for
- Nostr-sdk [https://github.com/rust-nostr/nostr]
- regex [https://crates.io/crates/regex]
- chronos [https://crates.io/crates/chrono]

To run this:
1. You'll need Rust & Cargo installed on your system. To check if Rust/Cargo is installed on a Terminal/Command Prompt type
        
        rustc --version
        cargo --version
        
   You should see some version numbers like 1.67.3 if they're installed

2. Please download the files in the repository 

3. In *main.rs* in src folder, please update your actual private key in nsec1... (ie. Bech32) format instead of the placeholder "Your secret key" in line 11
   (Will recommed using a dummy key-pair generated only for test purposes. You can generate new key-pairs on any Nostr-client[https://www.nostr.net/])

4. Go to this downloaded repository on Terminal/Command Prompt
       
       cd <Path where this repo was downloaded>/nostr-music-recos
       
5. Compile and run the program 
       
       cargo run


