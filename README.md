# OpenGOAL Launcher

Our attempt at distributing the [OpenGOAL](https://github.com/open-goal/jak-project) releases in a cross-platform and easy to use and update way. It also is a place for features involving the games, such as texture pack or mod management.

The launcher uses the [Tauri](https://tauri.app/) framework, if you are interested in our motivation for _why_ see below.

## Usage

See the [documentation on our website](https://opengoal.dev/docs/usage/installation/) for hopefully up to date instructions on how to use it.

## Development

Tauri requires a valid Rust installation, as well as a valid NodeJS installation.

For installing Rust, it's recommended to follow the instructions here https://www.rust-lang.org/tools/install

### Windows

```bash
scoop install nodejs
npm install -g yarn
```

### Linux (Ubuntu 22.04)

```bash
TODO
```

### Building and Running

To build and run the application locally, all you have to do is run:

```bash
yarn install
yarn tauri dev
```

### Code Overview

TODO

### References

- https://tauri.app/v1/guides/features/
- https://tauri.app/v1/api/js/
- https://svelte.dev/docs

## Why Tauri?

The gut reaction from many when looking at the launcher is _ugh, another Electron app_. This however is not the case. Tauri leverages typical HTML/CSS/JS for rendering the frontend -- but it does not do so by bundling Chromium. Instead it leverages the native WebView providers found on modern operating systems. This is also why the distribution is quite small (majority of the download size is for fonts/images/videos).

Here's a non-exhaustive list of all the benefits we get out of the box with Tauri that we'd have to build ourselves / straight-up not have available to us if we went with a non-electron GUI application framework.

- A built-in updater with private key signing
- Bundling scripts for MSI installers, AppImages, DMGs
- Essentially no differences frontend-wise across all operating systems
- No need to ship an interpreter (ie. PyQt)
- Typical web UI workflows that many people are familiar with
- The ability to painlessly write application logic in Rust
- Plethora of frontend E2E testing frameworks -- most of these are non-existant or cost money for other frameworks like Qt
