import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";

interface ActiveGameState {
  game?: SupportedGame;
}

// https://svelte.dev/docs/svelte/stores#When-to-use-stores
export const activeGameState: ActiveGameState = $state({
  game: undefined
});