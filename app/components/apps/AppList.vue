<script setup lang="ts">
import AppEntry from "~/components/apps/AppEntry.vue";

defineProps<{ apps: App[] }>()

let settings = await useSettings()

function onSetState(app: App, newState: boolean) {
  if (newState) {
    settings.value.apps.push(app)
  } else {
    let index = settings.value.apps.findIndex(value => value.name == app.name);
    settings.value.apps.splice(index, 1)
  }
}

</script>

<template>
  <AppEntry :key="app.name" v-for="app of apps" :app="app" :is-favorite="settings.apps.includes(app)"
            @setState="(newState) => onSetState(app, newState)"/>
</template>

<style scoped>

</style>