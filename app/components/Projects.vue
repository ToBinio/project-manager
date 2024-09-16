<script setup lang="ts">
let settings = await useSettings()
let projects = await useProjects(settings);

let searchText = ref("")

let filteredProjects = computed(() => {
  return projects.value
      .filter(project => project.name.toUpperCase().includes(searchText.value.toUpperCase()))
})
</script>

<template>
  <div class="bg-zinc-900 fixed w-full z-50">
    <SearchInput v-model:text="searchText"/>
  </div>

  <div class="flex flex-col gap-3 mt-12">
    <ProjectRunner v-for="project of filteredProjects" :key="project.name" :project="project"/>
  </div>
</template>

<style scoped>

</style>