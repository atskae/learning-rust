# Rusty-Journal

Notes from following this tutorial: https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/2-application-outline

# Notes

# Project Structure
```
rusty-journal/src
    * cli.rs <--- handles user input
    * main.rs <--- entry point into app
```

## Crates
* `structopt` parse command line arguments
* `chrono` timestamps, dates
* `anyhow` pretty-print errors
    * Catches errors from `?;` and pretty-prints them
    * `anyhow!` macro
