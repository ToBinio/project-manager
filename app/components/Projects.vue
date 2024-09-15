<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import SmallAppEntry from "~/components/apps/SmallAppEntry.vue";

let settings = await useSettings()
let projects = await useProjects(settings);

async function onRunProject(project: Project, app: App) {
  await invoke("run_project", {project: project, app: app})
}

function addToUsed(project: Project, app: App) {
  project.used.push(app.name)
}

function removeFromUsed(project: Project, app: App) {
  project.used.splice(project.used.indexOf(app.name), 1);
}
</script>

<template>
  <div :key="project.name" v-for="project of projects" class="p-1">
    <h4> {{ project.name }} </h4>

    <div class="flex gap-1">
      <div v-for="app of settings.apps">
        <button v-if="project.used.includes(app.name)"
                @click="() => onRunProject(project, app)"
                :key="app.name"
                class="hover:scale-125 transition">
          <SmallAppEntry :app="app"/>
        </button>
        <button v-if="project.used.includes(app.name)"
                @click="() => removeFromUsed(project, app)">del
        </button>
      </div>
    </div>

    <select v-model="project.selected" @change="() => {
        addToUsed(project, project.selected);
        onRunProject(project, project.selected);
      }">
      <option disabled selected>Select an app</option>

      <template v-for="app of settings.apps" :key="app.name">
        <option v-if="!project.used.includes(app.name)" :value="app">{{ app.name }}</option>
      </template>
    </select>
  </div>
</template>

<style scoped>
select {
  color: black;
}
</style>