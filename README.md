# Chart.js types API in Rust

***In Alpha, most likely does not work.***

## What this repo does
- "Transpiles" the Chart.js typescript types to Rust structs
- Also provide a rust crate/library for API use within WASM

## Dependencies
For transpiling:
- `npm` accessible on the $PATH

For crate use:
- a Rust project

## How to use this repo
- To transpile the code yourself run `regenerate_types.sh`.

 - If you just wish to include this crate in your rust project then use accordingly:
    ```toml
    [dependencies]
    chart-js-rs = { version = "0.0.1", git = "https://github.com/Billy-Sheppard/chart-js-rs.git" }
    ```

## How the "transpiling" works
1. Clones the Chart.js repo and checks out the most recent tag.
2. Installs TypeConv and QuickyType temorarily via `npm`.
3. Adjusts the `tsconfig.json` to work with TypeConv properly
4. Runs TypeConv to a `json` file per `.ts`.
5. Runs QuickType on those `json` files to `api/src/types/{file}.rs`.
6. Formats the `.rs` files.
7. Cleans up working directory.