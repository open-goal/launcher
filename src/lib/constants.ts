// TODO: update setup status messages to use typescript
export const SETUP_SUCCESS = {
  awaitingISO: { status: "Awaiting ISO File", percent: 0 },
  extractingISO: {
    status: "Extracting and Validating ISO contents",
    percent: 25,
  },
  decompiling: { status: "Decompiling the game", percent: 50 },
  compiling: { status: "Compiling the game", percent: 75 },
  ready: { status: "Ready to Play!", percent: 100 },
};

export const SETUP_ERROR = {
  noISO: { status: "No ISO File Selected!", percent: -1 },
  unsupportedOS: { status: "Unsupported OS!", percent: -1 },
};

export const enum SupportedGame {
  Jak1 = "Jak 1",
  Jak2 = "Jak 2",
  Jak3 = "Jak 3",
  JakX = "Jak X",
}
