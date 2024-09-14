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

function removeFromUsed(project, app) {
  project.used = project.used.filter((usedApp) => usedApp.name !== app.name);
}
</script>

<template>
  <div :key="project.name" v-for="project of projects" class="p-1">
    <h4> {{ project.name }} </h4>

    <div class="flex gap-1">
      <div v-for="app of settings.apps">
        <button v-if="project.used.includes(app.name)" @click="() => addToUsed(project, app)" :key="app.name"
                class="hover:scale-125 transition">
          <SmallAppEntry :app="app"/>
        </button>
      </div>
    </div>

    <div>
      <select @change="(event) => {
        let app = JSON.parse(event.target.value);

        addToUsed(project, app);
        onRunProject(project, app);
      }">
        <option disabled selected>Select an app</option>
        <option :disabled="project.used.includes(app.name)" v-for="app of settings.apps" :key="app.name"
                :value="JSON.stringify(app)">
          {{ app.name }}
        </option>
      </select>
    </div>
  </div>
</template>

<style scoped>
select {
  color: black;
}
</style>