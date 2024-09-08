<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {useSettings} from "../composable/useSettings.ts";

let settings = await useSettings()
let names = await invoke<String[]>("get_projects", {settings: settings.value});
</script>

<template>
  <div :key="name" v-for="name of names">
    {{ name }}
  </div>
</template>

<style scoped>

</style>