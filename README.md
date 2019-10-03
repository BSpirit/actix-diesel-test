# Project setting

If you don't have rust installed:
```
curl https://sh.rustup.rs -sSf | sh
```

If you don't have diesel cli installed:
```
cargo install diesel_cli --no-default-features --features sqlite
```

Set up database:
```
diesel setup
```

# Run app

At the root of the project:
```
cargo run
```

The app listen on port 8088