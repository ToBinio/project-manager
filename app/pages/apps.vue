<script setup lang="ts">
import {convertFileSrc} from "@tauri-apps/api/core";

let apps = await useApps()

let searchText = ref("")

let filteredApps = computed(() => {
  return apps.value
      .sort((a, b) => a.name.localeCompare(b.name))
      .filter(app => app.name.toUpperCase().includes(searchText.value.toUpperCase()))
})



</script>

<template>
  <input class="border-2 rounded-md mx-5 my-3" type="text" v-model="searchText">

  <div :key="app.name" v-for="app of filteredApps" class="mx-5 py-2">
    <img :src="convertFileSrc(app.icon_path)">
    <h2 class="font-bold"> {{ app.name }}</h2>
    <div> {{ app }}</div>
  </div>
</template>

<style scoped>

</style>