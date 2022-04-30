# Exercism

## Setup

1. `cd exercism`
2. `./exercism configure -w .`
3. Go to [Exercism settings page](https://exercism.org/settings) -> `API/CLI` and copy token
4. Add copied token to cli `./exercism configure --token=<token>`
5. Find task to solve at [exercism](https://exercism.org)
5. Copy cli command to download exercise
6. Run `cd rust/<task_name>`, solve task and run tests
7. Then from the `exercism` dir run `./exercism submit <files>` and put files with solutions (usually it is `src/lib.rs` and optional `Cargo.toml` if you have been added some dependencies)