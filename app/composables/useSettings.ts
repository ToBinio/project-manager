import {invoke} from "@tauri-apps/api/core";
import {ref, watch} from "vue";

export type Settings = {
    path: String | null
    apps: App[]
}

export async function useSettings() {
    let settings = useState<Settings>("settings");

    await callOnce(async () => {
        settings.value = await invoke("get_settings")
    })

    watch(settings, async (value) => {
        await invoke("save_settings", {settings: value})
    }, {deep: true});

    return settings
}