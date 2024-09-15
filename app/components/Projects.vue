<script setup lang="ts">
let settings = await useSettings()
let projects = await useProjects(settings);
</script>

<template>
  <div class="flex flex-col gap-3">
    <ProjectRunner v-for="project of projects" :key="project.name" :project="project"/>
  </div>
</template>

<style scoped>

</style>