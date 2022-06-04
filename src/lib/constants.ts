export const SETUP_SUCCESS = {
  avxSupported: { status: "AVX SUPPORTED", percent: 10 },
  openGLSupported: { status: "OPENGL SUPPORTED", percent: 20 },
  checkCompatible: { status: "Checking Compatibility", percent: 0 },
  awaitingISO: { status: "Awaiting ISO File", percent: 20 },
  extractingISO: {
    status: "Extracting and Validating ISO contents",
    percent: 40,
  },
  decompiling: { status: "Decompiling the game", percent: 60 },
  compiling: { status: "Compiling the game", percent: 80 },
  ready: { status: "Ready to Play!", percent: 100 },
};

export const SETUP_ERROR = {
  unsupportedAVX: { status: "UNSUPPORTED AVX", percent: -1 },
  noISO: { status: "No ISO File Selected!", percent: -1 },
  unsupportedOS: { status: "Unsupported OS!", percent: -1 },
  unsupportedOpenGL: { status: "UNSUPPORTED OPENGL VERSION", percent: -1 },
};

export const SUPPORTED_GAME = {
  Jak1: "Jak 1",
  Jak2: "Jak 2",
  Jak3: "Jak 3",
  JakX: "Jak X",
};
