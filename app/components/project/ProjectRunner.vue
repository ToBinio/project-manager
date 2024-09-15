<script setup lang="ts">
import AppIcon from "~/components/apps/AppIcon.vue";
import {invoke} from "@tauri-apps/api/core";

const props = defineProps<{ project: Project }>()

let settings = await useSettings()

const apps = computed(() => {
  return settings.value.apps.filter((app) => {
    return props.project.used.includes(app.name);
  })
})

async function onRunProject(app: App) {
  await invoke("run_project", {project: props.project, app: app})
}

function addToUsed(app: App) {
  if (props.project.used.includes(app.name)) return

  if (props.project.used.length >= 3) {
    props.project.used.splice(0, 1)
  }

  props.project.used.push(app.name)
}
</script>

<template>
  <div class="p-2 bg-zinc-800 rounded-lg mx-2">

    <h4 class="text-xl bold mb-2">
      {{ project.name }}
    </h4>

    <div class="flex gap-1">
      <button v-for="app of apps" :key="app.name" class="text-white rounded-lg hover:scale-110 transition"
              @click="() => onRunProject(app) ">
        <AppIcon :app="app"/>
      </button>
      <div class="group relative hover:z-10">
        <button
            class="h-8 w-8 bg-zinc-600 rounded-lg hover:scale-110 transition flex items-center justify-center">
          <Icon mode="svg" name="ph:dots-three-vertical-bold" size="24"/>
        </button>

        <div class="group-hover:block hidden absolute w-max">
          <div class="mt-2 bg-zinc-600 p-2 rounded-lg flex flex-col gap-2">
            <button v-for="app of settings.apps" :key="app.name" :value="app" @click="() => {
                addToUsed(app)
                onRunProject(app)
            }">
              <div class="flex gap-2 hover:scale-105 transition">
                <AppIcon :app="app"/>
                <div>
                  {{ app.name }}
                </div>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>