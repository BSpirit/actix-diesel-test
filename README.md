# Project setting

If you don't have diesel cli installed:
```
cargo install diesel_cli --no-default-features --features sqlite
```

Set up database:
```
diesel setup
```

Run migrations:
```
diesel migration run
```

# Run app

At the root of the project:
```
cargo run
```

The app listen on port 8080