import { vi, beforeAll } from "vitest";

beforeAll(() => {
  window.__TAURI_IPC__ = vi.fn();
});
