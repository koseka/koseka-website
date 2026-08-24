# Contribution Guide

This file is primarily intended for developers who wish to fork the project and potentially contribute to it. This project uses [Phased Versioning](https://phased-versioning.koseka.net), which defines the versioning, branching, and release rules, and commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. So, make sure to read both first before contributing to the project in any way.

> **A note on the name "Trunk":** this project uses two unrelated tools that are both called Trunk. The [Trunk WASM bundler](https://trunkrs.dev) (a cargo binary) builds and serves the Sycamore client, and the [Trunk Code Quality CLI](https://docs.trunk.io/code-quality/overview) (an npm package) is the metalinter that formats and lints the code. The instructions below always specify which one is meant.

## Project Structure

Here are the main directories and files in the project:

```plaintext
.
├── assets/
├── client/
│   ├── assets/
│   ├── src/
│   ├── index.html
│   ├── Trunk.toml
│   └── Cargo.toml
├── server/
│   ├── src/
│   └── Cargo.toml
├── img/
└── package.json
```

The `client/` directory contains the [Sycamore](https://sycamore.dev) web app, compiled to WebAssembly by the Trunk WASM bundler and styled with [Tailwind CSS](https://tailwindcss.com). Its `client/assets/` subdirectory holds the curated assets that ship with the site, while the top-level `assets/` directory is the raw asset library. The `server/` directory contains the Rust server that hosts the built site, and `img/` holds the graphics used by the README.

Additionally, the `package.json` file configures the [Node](https://nodejs.org) environment required to run the [Trunk Code Quality CLI](https://docs.trunk.io/code-quality/overview) metalinter.

## Setting Up the Development Environment

First, ensure that you have the latest version of **Rust** installed on your machine, along with the `wasm32-unknown-unknown` target and the Trunk WASM bundler. You can install **Rust** by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install), and then run:

```sh
rustup target add wasm32-unknown-unknown                    # Adds the WASM compilation target.
cargo install trunk                                         # Installs the Trunk WASM bundler.
```

Second, this project uses the [**Trunk Code Quality CLI**](https://docs.trunk.io/code-quality/overview) as an npm package for formatting and linting the code, and **npm** as a package manager. So, make sure you have **node.js** and **npm** installed on your machine. You can install both of them from the official [node.js](https://nodejs.org) website.

Next, clone the `koseka-website` repository to your local machine and install the development dependencies:

```sh
git clone https://github.com/koseka/koseka-website.git      # Clones the repository.
cd koseka-website                                           # Moves into the project directory.
npm install                                                 # Installs the development dependencies.
```

Since the **Trunk Code Quality CLI** is used for formatting the code, it's best if you disable the _format on save_ option in your editor to avoid potential conflicts with the project's formatting configurations.

If you are using [**Zed**](https://zed.dev), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.zed/settings.json` file at the root of the project directory:

```json
{
  "format_on_save": "off"
}
```

If you are using [**VSCode**](https://code.visualstudio.com), you can locally disable the _format on save_ option of your editor for this project by adding the following line to the `.vscode/settings.json` file at the root of the project directory:

```json
{
  "editor.formatOnSave": false
}
```

As for the linting, the project comes with its own linters and configurations, so if you have your own linters installed with custom configurations, you should make sure they don't conflict with the project's linters. You can check the list of linters (and formatters) along with their configurations in the `.trunk/trunk.yaml` file and the `.trunk/configs/` directory.

If you have followed all the steps correctly, you should now have a working development environment for the project. If you encounter any issues, feel free to open an issue on the project's [GitHub repository](https://github.com/koseka/koseka-website/issues).

## Linting and Formatting the Code

The linters and formatters work through git hooks, so they will run automatically when you commit changes. However, it's best to also run them manually before committing changes to avoid failing the commit hook.

To make sure the **Trunk Code Quality CLI** is managing the git hooks, you can run the following command:

```sh
npm run trunk git-hooks sync
```

You can manually run the linters and formatters using the following commands:

```sh
npm run check                                               # Runs linters and formatters on all the changed files.
npm run check --all                                         # Runs linters and formatters on all the files in the repository.
```

You can manually format the code using the following commands:

```sh
npm run fmt                                                 # Formats all the changed files.
npm run fmt --all                                           # Formats all the files in the repository.
```

## Building and Serving the Site

You can build and serve the client using the following commands (run from the `client/` directory, using the Trunk WASM bundler):

```sh
trunk serve                                                 # Builds the client and serves it on http://localhost:8080 with auto-reload.
trunk build                                                 # Builds the client into `client/dist/` in debug mode.
trunk build --release                                       # Builds the client into `client/dist/` in release mode.
```

You can build and run the server using the following commands (run from the `server/` directory):

```sh
cargo run                                                   # Builds and runs the server in debug mode.
cargo build --release                                       # Builds the server in release mode.
```

## License

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
