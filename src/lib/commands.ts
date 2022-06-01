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

export async function openDir(dir) {
  try {
    return await invoke("open__dir", { dir });
  } catch (e) {
    console.log(e);
  }
}
