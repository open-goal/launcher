export const enum SupportedGame {
  Jak1 = "Jak 1",
  Jak2 = "Jak 2",
  Jak3 = "Jak 3",
  JakX = "Jak X",
}

export function fromRoute(gameName: string): SupportedGame {
  switch (gameName) {
    case "jak1":
      return SupportedGame.Jak1;
    case "jak2":
      return SupportedGame.Jak2;
    case "jak3":
      return SupportedGame.Jak3;
    case "jakx":
      return SupportedGame.JakX;
  }
}

export function getInternalName(activeGame: SupportedGame): string {
  switch (activeGame) {
    case SupportedGame.Jak1:
      return "jak1";
    case SupportedGame.Jak2:
      return "jak2";
    case SupportedGame.Jak3:
      return "jak3";
    case SupportedGame.JakX:
      return "jakx";
  }
}
