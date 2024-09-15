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
  <div class="flex flex-col justify-center items-center min-h-screen bg-gray-1000">
    <div v-for="project of projects" :key="project.name"
         class="max-w-md w-full p-6 bg-zinc-700 shadow-lg rounded-lg mb-3 mt-3">

      <h4 class="text-xl font-semibold text-gray-100 mb-4 border-b border-gray-1000 pb-3">
        {{ project.name }}
      </h4>

      <div class="flex flex-col mb-6">
        <div v-for="app of settings.apps" :key="app.name" class="flex items-center space-x-2">
          <button v-if="project.used.includes(app.name)"
                  @click="() => onRunProject(project, app)"
                  class="p-2 bg-blue-500 text-white rounded-lg hover:scale-105 hover:bg-blue-600 transition-transform">
            <SmallAppEntry :app="app"/>
          </button>

          <button v-if="project.used.includes(app.name)"
                  @click="() => removeFromUsed(project, app)"
                  class="px-3 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors">
            🗙
          </button>
        </div>
      </div>

      <select v-model="project.selected"
              class="w-full p-3 border border-gray-1000 rounded-lg shadow-sm bg-zinc-600 text-gray-100 focus:outline-none focus:ring focus:ring-blue-700"
              @change="() => {
              addToUsed(project, project.selected);
              onRunProject(project, project.selected);

              project.selected = 'Select an app'
            }">
        <option disabled>Select an app</option>

        <template v-for="app of settings.apps" :key="app.name">
          <option v-if="!project.used.includes(app.name)" :value="app">{{ app.name }}</option>
        </template>
      </select>
    </div>
  </div>

</template>

<style scoped>

</style>