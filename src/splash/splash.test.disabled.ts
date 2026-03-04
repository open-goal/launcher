import {
  render,
  waitFor,
  screen,
  cleanup,
  fireEvent,
} from "@testing-library/svelte";
import Splash from "./Splash.svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";

vi.mock("$lib/utils/file-dialogs");

describe("Splash.svelte", () => {
  // TODO: @testing-library/svelte claims to add this automatically but it doesn't work without explicit afterEach
  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
    vi.resetAllMocks();
  });

  it("should render the splash", async () => {
    mockIPC((cmd, args) => {
      console.log(`Unhandled Tauri IPC: ${cmd}`);
    });
    render(Splash, {});
    await waitFor(() => {
      expect(screen.getByTestId("splash-logo")).toBeTruthy();
    });
  });

  it("should display the locale dropdown", async () => {
    // TODO - generalize into function
    mockIPC((cmd, args) => {
      if (cmd === "get_setting_value" && args.key === "locale") {
        return null;
      } else {
        console.log(`Unhandled Tauri IPC: ${cmd}`);
      }
    });
    render(Splash, {});
    const localeSelect = await screen.findByTestId("locale-select");
    expect(localeSelect).toBeTruthy();
  });

  it("should set the locale when selected", async () => {
    // TODO - generalize into function
    mockIPC((cmd, args) => {
      if (cmd === "get_setting_value" && args.key === "locale") {
        return null;
      }
      if (cmd === "update_setting_value" && args.key === "locale") {
        return;
      } else {
        console.log(`Unhandled Tauri IPC: ${cmd}`);
      }
    });
    render(Splash, {});
    const localeSelect = (await screen.findByTestId(
      "locale-select",
    )) as HTMLSelectElement;
    expect(localeSelect).toBeTruthy();
    fireEvent.change(localeSelect, { target: { value: "en-US" } });
    expect(localeSelect.value).toBe("en-US");
  });
});
