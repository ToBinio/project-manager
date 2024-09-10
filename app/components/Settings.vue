<script setup lang="ts">
import {open} from "@tauri-apps/plugin-dialog";
import AppList from "~/components/apps/AppList.vue";

let settings = await useSettings()

async function onChangePath() {
  let path = await open({multiple: false, directory: true});

  if (path)
    settings.value.path = path;
}
</script>

<template>
  <h1>Settings</h1>
  current Path: {{ settings.path }}
  <button @click="onChangePath">change Path</button>

  <div>
    <h3>Apps</h3>
    <AppList :apps="settings.apps"/>
  </div>
</template>

<style scoped>
</style>