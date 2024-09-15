import {invoke} from "@tauri-apps/api/core";
import {ref, watch} from "vue";
import type {Settings} from "~/composables/useSettings";

export type Project = {
    name: String
    path: String
    used: String[]
    selected: String
}

export async function useProjects(settings: Ref<Settings>) {
    let projects = useState<Project[]>("projects");

    await callOnce(async () => {
        projects.value = await invoke("get_projects", {settings: settings.value})

        watch(settings, async value => {
            projects.value = await invoke("get_projects", {settings: settings.value})
        }, {deep: true})
    })

    projects.value.forEach(value => {
        value.used = [];
        value.selected = "Select an app"
    })

    return projects
}