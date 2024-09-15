<script setup lang="ts">

import {convertFileSrc} from "@tauri-apps/api/core";

defineProps<{ app: App }>()

</script>

<template>
  <div>
    <img v-if="app.icon_path" class="w-8" :src="convertFileSrc(app.icon_path)" :alt="app.name">
    <div v-else>
      {{ app.name }}
    </div>
  </div>
</template>

<style scoped>

</style>