import { invoke_rpc } from "./rpc";

export async function refreshModSources(): Promise<void> {
    return await invoke_rpc("refresh_mod_sources", {}, () => { });
}