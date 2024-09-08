<script setup lang="ts">
import {useSettings} from "~/composables/useSettings.ts";
import {open} from "@tauri-apps/plugin-dialog";

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
</template>

<style scoped>
</style>