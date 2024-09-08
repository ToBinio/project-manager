import {invoke} from "@tauri-apps/api/core";
import {ref, watch} from "vue";

type Settings = {
    path: String | null
}

export async function useSettings() {
    let settings = ref(await invoke<Settings>("get_settings"));

    watch(settings, async (value) => {
        await invoke("save_settings", {settings: value})
    }, {deep: true});

    return settings
}