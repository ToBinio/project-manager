<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import SmallAppEntry from "~/components/apps/SmallAppEntry.vue";

let settings = await useSettings()
let projects = await useProjects(settings);

async function onRunProject(project: Project, app: App) {
  await invoke("run_project", {project: project, app: app})
}
</script>

<template>
  <div :key="project.name" v-for="project of projects" class="p-1">
    <h4> {{ project.name }} </h4>

    <div class="flex gap-1">
      <button @click="() => onRunProject(project, app)" :key="app.name" v-for="app of settings.apps" class="hover:scale-125 transition">
        <SmallAppEntry :app="app"/>
      </button>
    </div>
  </div>
</template>

<style scoped>

</style>