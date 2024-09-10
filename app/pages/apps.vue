<script setup lang="ts">
import AppList from "~/components/apps/AppList.vue";

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

  <AppList :apps="filteredApps"/>
</template>

<style scoped>

</style>