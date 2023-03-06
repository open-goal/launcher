import { TranslatedStrings } from "./translations/translations";

export const enum SupportedGame {
  Jak1 = "Jak 1",
  Jak2 = "Jak 2",
  Jak3 = "Jak 3",
  JakX = "Jak X",
}

// TODO - we should really just have `SupportedGame` be a class instead of an enum
// then these could just be methods
export function getGameTitle(game: SupportedGame) {
  switch (game) {
    case SupportedGame.Jak1:
      return TranslatedStrings.jak1_gameName;
    default:
      return "";
  }
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
