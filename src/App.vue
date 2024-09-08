<script setup lang="ts">
import Settings from "./components/Settings.vue";
import Projects from "./components/Projects.vue";

</script>

<template>
  <Suspense>
    <Settings/>
  </Suspense>
  <Suspense>
    <Projects/>
  </Suspense>
</template>

<style scoped>
</style>
