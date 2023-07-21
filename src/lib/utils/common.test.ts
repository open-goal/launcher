import { describe, expect, it } from "vitest";
import { isInDebugMode } from "./common";

describe("isInDebugMode", () => {
  it("should return true when in debug mode", async () => {
    process.env["NODE_ENV"] = "development";
    expect(isInDebugMode()).toBeTruthy();
  });

  it("should return true when in debug mode", async () => {
    process.env["NODE_ENV"] = "not-development";
    expect(isInDebugMode()).toBeFalsy();
  });
});
