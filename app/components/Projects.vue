<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";

let settings = await useSettings()
let projects = await useProjects(settings);

async function onRunProject(project: Project, app: App) {
  await invoke("run_project", {project: project, app: app})
}
</script>

<template>
  <div :key="project.name" v-for="project of projects" class="p-1">
    {{ project.name }}

    <button @click="() => onRunProject(project, app)" :key="app.name" v-for="app of settings.apps"
            class="bg-gray-400 hover:bg-gray-300">
      {{ app.name }}
    </button>
  </div>
</template>

<style scoped>

</style>