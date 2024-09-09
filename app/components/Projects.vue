<script setup lang="ts">
let settings = await useSettings()
let projects = await useProjects(settings);
</script>

<template>
  <div :key="name" v-for="name of projects" class="ml-5">
    {{ name }}
  </div>
</template>

<style scoped>

</style>