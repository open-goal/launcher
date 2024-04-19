import type { ModSourceData } from "./bindings/ModSourceData";
import { invoke_rpc } from "./rpc";

export async function refreshModSources(): Promise<void> {
    return await invoke_rpc("refresh_mod_sources", {}, () => { });
}

export async function getModSourcesData(): Promise<Record<string, ModSourceData>> {
    return await invoke_rpc("get_mod_sources_data", {}, () => {
        let val: Record<string, ModSourceData> = {};
        return val;
    });
}