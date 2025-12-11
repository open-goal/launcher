import type { SupportedGame } from "../SupportedGame";

const GAME_MAP: Record<SupportedGame, true> = {
    jak1: true,
    jak2: true,
    jak3: true,
    jakx: true,
};

export function toSupportedGame(value: string | undefined): SupportedGame | undefined {
    if (!value) {
        return undefined;
    }
    return value in GAME_MAP ? (value as SupportedGame) : undefined;
}
