import { invoke } from "@tauri-apps/api/tauri";

export async function getHighestSimd() {
  try {
    return await invoke("get_highest_simd");
  } catch (e) {
    if (e === "AVXNotSupported") {
      return "noavx";
    } else {
      return undefined;
    }
  }
}
