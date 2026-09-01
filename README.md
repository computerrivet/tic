# tic

A minimal terminal task manager written in Rust.

`tic` stores your tasks as plain JSON in a local `.config/tic` directory, so it
works without a database, a server, or an account. Add tasks, mark them done,
edit or remove them, and see how long ago each one was created — all from a few
simple commands.

## Installation

Build and install with [Cargo](https://doc.rust-lang.org/cargo/):

```sh
cargo install --path .
```

This installs the `tic` binary into your Cargo `bin` directory.

## Usage

```sh
tic add "write the report"      # Add a task
tic list                        # List all tasks
tic done 1                      # Mark task #1 as done
tic edit 1 "send the email"     # Update task #1's description
tic remove 2                    # Remove task #2
tic clear                       # Remove all tasks
```

`tic list` renders each task with a checkbox, id, description, and relative
age:

```
[ ] #1 write the report (2 min ago)
[x] #2 send the email (1 hr ago)
```

### Storage

By default, tasks are stored in `.config/tic/tasks.json` relative to the
current directory. Use the global `--path` flag to store and read tasks from a
different directory:

```sh
tic --path ~/work list
```

More usage details are available via `tic --help` and `tic list --help`.

## Development

Run the test suite with:

```sh
cargo test
```

## License

Apache 2.0
