# OpenGOAL Launcher

## Description

A launcher for users to install and run the OpenGOAL project with ease

## Preview

![Launcher Preview](https://user-images.githubusercontent.com/86533397/193725375-a75dbeb0-d9f3-4e14-bf67-d872f89d11c2.png)

## Disclaimer

Users are required to provide their own copy of the ISO file in order to run the game.

## Features

- [x] Automatic Updates
- [x] Windows Support
- [x] Linux Support
- [ ] Mac Support (TBD -- Not Planned)
- [ ] Texture Pack Management (Soon)
- [ ] Mod Management

## Resources

- [OpenGOAL Github Organization](https://github.com/open-goal/)
- [OpenGOAL Documentation](https://opengoal.dev/)
- [OpenGOAL Discord](https://discord.gg/twBEFbMnqw)

## Development

We are using Tauri to build a native app, but still with simple Web technology. You will need to setup the prerequesites using the instructions here https://tauri.app/v1/guides/getting-started/prerequisites/

> Additionally, this presumes your environment has WebView2 (windows) or webkit2 (linux) already available. This is a requirement for end-users as well! Many modern OSes already ship with such a thing, but it's something we'll need to investigate.

- `yarn install`
- `yarn tauri dev`

This builds the app with Tauri (this is a rust compilation, the first run will take a while) and the frontend is served via Vite -- a fast alternative to webpack that offers HMR.
