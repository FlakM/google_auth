# About

This is a simple project for lazy people like me.
Grabbing your phone is so 2019...

# Installation

Get rust tooling using  https://rustup.rs/ And run:

```bash
cargo install --path .

# binary will be installed in $HOME/.cargo/bin/google_auth

export GOOGLE_SECRET="putyoursecrethere"

code=$(google_auth)
echo "got code $code 😎"
``` 