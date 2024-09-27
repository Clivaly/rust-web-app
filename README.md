## Rust Axum for Production.

### Dev (watch)

> NOTE: Install cargo watch with `cargo install cargo-watch`.
```sh
# Terminal 1 - To run the server.
$ cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the quick_dev.
$ cargo watch -q -c -w examples/ -x "run --example quick_dev"

# Terminal 2 (Optional) - To run the quick_dev by "tests" folder.
$ cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

# Run .cargo
$ cargo watch -q -c -w src/ -w .cargo/ -x "run"
```

### Unit Test (watch)

```sh
$ cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
$ cargo watch -q -c -x "test model::task::tests -- --nocapture"
```

### Dev

```sh
# Terminal 1 - To run the server.
$ cargo run

# Terminal 2 - To run the tests.
$ cargo run --example quick_dev
```

### Unit Test

```sh
$ cargo test -- --nocapture

$ cargo watch -q -c -x test model::task::tests::test_create -- --nocapture
```

### Staring DataBase by docker.

```sh
# Start postgresql server docker image:
$ docker run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
$ docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
$ ALTER DATABASE postgres SET log_statement = 'all';
```

### Staring DataBase by docker-compose.

```sh
$ docker-compose up -d --wait

# Enter postgres inside container:
$ docker-compose exec database /bin/bash
# Connect:
$ psql -U postgres -q -d postgres
$ \c app_db
$ \d
$ select * from "user";
$ \d task

# (optional) For pg to print all sql statements.
# In psql command line started above.
$ ALTER DATABASE postgres SET log_statement = 'all';
```

## AwesomeApp rust-web-app

More info at: https://awesomeapp.dev/rust-web-app/

Credits to **Jeremy Chone**,
check your [YouTube Channel.](https://www.youtube.com/@JeremyChone)


