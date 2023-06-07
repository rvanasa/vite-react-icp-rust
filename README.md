# Vite + React + IC (Rust)

This project template gives you everything you need to build a Web3 application on the [Internet Computer](https://internetcomputer.org/).

Check out [Vite + React + Motoko](https://github.com/rvanasa/vite-react-motoko) for a beginner-friendly starter project with a [Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko) backend. 

## Create a New Project

Make sure that [Node.js](https://nodejs.org/en/) `>= 16.x`, [`dfx`](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) `>= 0.12.x`, and [Rust](https://www.rust-lang.org/tools/install) are installed on your system.

After installing Rust, run these commands to configure your system for IC canister development:

```sh
rustup target add wasm32-unknown-unknown # Required for building IC canisters
cargo install cargo-watch # Optional; used for live reloading in `npm start`
```

Next, run the following commands in a new, empty project directory:

```sh
npx degit rvanasa/vite-react-ic-rust # Download this starter project
dfx start --clean --background # Run dfx in the background
npm run setup # Install packages, deploy canisters, and generate type bindings

npm start # Start the development server
```

When ready, run `dfx deploy` to build and deploy your application.

## Technology Stack

- [Vite](https://vitejs.dev/): high-performance tooling for front-end web development
- [React](https://reactjs.org/): a component-based UI library
- [TypeScript](https://www.typescriptlang.org/): JavaScript extended with syntax for types
- [Sass](https://sass-lang.com/): an extended syntax for CSS stylesheets
- [Prettier](https://prettier.io/): code formatting for a wide range of supported languages
- [Rust](https://www.rust-lang.org/): a fast, safe programming language for writing [Internet Computer](https://internetcomputer.org/) canisters

## Documentation

- [Vite developer docs](https://vitejs.dev/guide/)
- [React quick start guide](https://beta.reactjs.org/learn)
- [Internet Computer docs](https://internetcomputer.org/docs/current/developer-docs/ic-overview)
- [Rust developer docs](https://rustc-dev-guide.rust-lang.org/)
- [`dfx.json` reference schema](https://internetcomputer.org/docs/current/references/dfx-json-reference/)

## Tips and Tricks

- Customize your project's code style by editing the `.prettierrc` file and then running `npm run format`.
- Reduce the latency of update calls by passing the `--emulator` flag to `dfx start`.
- Split your frontend and backend console output by running `npm run frontend` and `npm run backend` in separate terminals.

## Run in your Browser

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/rvanasa/vite-react-ic-rust)

For a considerably faster setup time, check out the [Vite + React + Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko) starter project.
