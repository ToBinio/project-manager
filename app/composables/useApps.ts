import {invoke} from "@tauri-apps/api/core";

export type App = {
    name: String
    icon_path: String
}

export async function useApps() {
    let apps = useState<App[]>("apps");

    await callOnce(async () => {
        apps.value = await invoke("get_all_apps")
    })

    return apps
}