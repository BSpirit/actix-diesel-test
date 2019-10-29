# Local development and database setup

1 - If you don't have rust installed:
```
curl https://sh.rustup.rs -sSf | sh
```

2 - If you don't have diesel cli installed:
```
cargo install diesel_cli --no-default-features --features sqlite
```

3 - Set up database:
```
diesel setup
```

4 - Run migrations
```
diesel migration run
```

5 - To run app:
```
cargo run
```

The app listen on port 8088

# Using Docker

1 - Build binary using Docker image (Better workflow for crossplatform compilation, keep cache and avoid recompiling all of the dependencies)
```
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```

2 - Run docker container
```
docker-compose up -d
```

NB: Migrations should first be run from local machine (At the moment)

The app listen on port 8088

#### Other commands

Remove container using docker-compose
```
docker-compose down
```

Build docker image using given Dockerfile
```
docker build -t actix-diesel-test .
```

Run container without docker-compose
```
docker run -it --rm -p 8088:8088 actix-diesel-test
```
