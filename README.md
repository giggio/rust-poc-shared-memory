# Rust PoC for shared memory in a container

This is using shared memory in Rust, in a container.

## Building

Build with Cargo:

```bash
cargo build
```

## Running

Run the application:

```bash
cargo run
```

Options

- `struct` - will use a struct to store two single characters
- `array` - will use an array to store two integers
- `isinit` - will show if the shared memory is initialized or not

### Example

```bash
cargo run -- struct
```

Then run another one and see how the communicate.

Another option is to run the first one, and, within 5 seconds run the second one:

```bash
cargo run -- isinit
```

It will fail, as it waits for only one second, and there is a delay of 5 seconds to simulate a real delay in startup.
If you run it again after the first process is initialized it will succeed and and show that the shared memory is initialized.

## Running in containers

**Note**: The [docker-compose.yaml](./docker-compose.yaml) file is not supposed to run, it is only an example on docker run params
and it useful when building.

### Build image

Build the container image:

```bash
docker compose build
```

### Run container

Run the container:

```bash
#leader
docker run --rm -ti --ipc=shareable --name=pocshm_leader pocshm
# follower
docker run --rm -ti --ipc=container:pocshm_leader --name=pocshm_follower
```

Check if the shared memory is initialized:

```bash
# checking on the leader
docker exec -t pocshm_leader /pocshm isinit
# or checking from another container
docker run --rm -ti --ipc=container:pocshm_leader pocshm isinit
```

## Author

[Giovanni Bassi](https://links.giggio.net/bio)

## License

Licensed under the [MIT license](https://opensource.org/license/MIT).
