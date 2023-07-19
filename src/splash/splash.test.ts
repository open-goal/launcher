import { render, waitFor, screen, cleanup } from "@testing-library/svelte";
import Splash from "./Splash.svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";

describe("Splash.svelte", () => {
  // TODO: @testing-library/svelte claims to add this automatically but it doesn't work without explicit afterEach
  afterEach(() => cleanup());

  it("should render the splash", async () => {
    render(Splash, {});
    const logo = screen.getByTestId("splash-logo");
  });

  it("should display the locale dropdown", async () => {
    // TODO - generalize into function
    mockIPC((cmd, args) => {
      if (cmd === "get_locale") {
        return null;
      } else {
        console.log(`Unhandled Tauri IPC: ${cmd}`);
      }
    });
    render(Splash, {});
    await waitFor(() => {
      const localeSelect = screen.getByTestId("locale-select");
      expect(localeSelect).toBeTruthy();
    });
  });
});
