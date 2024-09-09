<script setup lang="ts">

</script>

<template>
  <div>
    <div class="bg-blue-950 text-white fixed h-12 w-full flex items-center gap-2 pl-2">
      <NuxtLink class="px-3" to="/">Projects</NuxtLink>
      <NuxtLink class="px-3" to="/settings">Settings</NuxtLink>
      <NuxtLink class="px-3" to="/apps">Apps</NuxtLink>
    </div>
    <div class="pt-12">
      <slot/>
    </div>
  </div>
</template>

<style scoped>
.router-link-active {
  @apply bg-blue-800  rounded-md
}
</style>