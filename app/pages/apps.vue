<script setup lang="ts">
import AppList from "~/components/apps/AppList.vue";

let apps = await useApps()

let searchText = ref("");
let filteredApps = computed(() => {
  return apps.value
      .sort((a, b) => a.name.localeCompare(b.name))
      .filter(app => app.name.toUpperCase().includes(searchText.value.toUpperCase()))
})
</script>

<template>
  <div class="bg-zinc-900 fixed w-full">
    <SearchInput v-model:text="searchText"/>
  </div>

  <div class="mt-9">
    <AppList :apps="filteredApps"/>
  </div>
</template>

<style scoped>
</style>