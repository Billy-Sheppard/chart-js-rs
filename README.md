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
2. Installs TypeDoc and QuickyType temorarily via `npm`.
3. Adjusts the `tsconfig.json` to work with TypeDoc properly
4. Runs TypeDoc to a `json` file.
5. Runs QuickType on that `json` file to `api/src/types.rs`.
6. Cleans up working directory.

