# OpenGOAL Launcher

[![Crowdin](https://badges.crowdin.net/opengoal-launcher/localized.svg)](https://crowdin.com/project/opengoal-launcher)

Our attempt at distributing the [OpenGOAL](https://github.com/open-goal/jak-project) releases in a cross-platform and easy to use and update way. It also is a place for features involving the games, such as texture pack or mod management.

The launcher uses the [Tauri](https://tauri.app/) framework, if you are interested in our motivation for _why_ see below.

- [Usage](#usage)
- [Asking for help](#asking-for-help)
- [Development](#development)
  - [Windows](#windows)
  - [Linux (Ubuntu 22.04)](#linux-ubuntu-2204)
  - [Building and Running](#building-and-running)
  - [Code Overview](#code-overview)
  - [References](#references)
- [Why Tauri?](#why-tauri)

## Usage

See the [documentation on our website](https://opengoal.dev/docs/usage/installation/) for hopefully up to date instructions on how to use it.

## Asking for help

When asking for help, please download the support package which includes logs to help someone diagnose the problem.

![](./docs/support-package.png)

If you cannot do this (for example, the bug relates to making the package / the application won't launch) then you can find the application logs in the following folders:

- Windows `C://Users/<YOUR_USER_NAME>/AppData/Roaming/OpenGOAL-Launcher/logs`
- Linux `/home/<YOUR_USER_NAME>/.config/OpenGOAL-Launcher/logs`

Note that both `AppData` and `.config` are hidden folders.

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
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev # tauri deps, see - https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash # installs Node Version Manager (ubuntus package is woefully out of date)
source ~/.bashrc
nvm install lts/hydrogen # installs latest nodejs 18.X
npm install -g yarn
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
- https://tailwindcss.com/
- https://flowbite-svelte.com/

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
