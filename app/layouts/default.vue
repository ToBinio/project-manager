<script setup lang="ts">

</script>

<template>
  <div class="text-gray-100">
    <div class="bg-zinc-900 fixed h-14 w-full flex items-center gap-3 pl-2 font-medium z-50">
      <NuxtLink class="px-4 py-0.5 bg-zinc-700 rounded-md hover:bg-zinc-600 transition" to="/">Projects</NuxtLink>
      <NuxtLink class="px-4 py-0.5 bg-zinc-700 rounded-md hover:bg-zinc-600 transition" to="/settings">Settings</NuxtLink>
      <NuxtLink class="px-4 py-0.5 bg-zinc-700 rounded-md hover:bg-zinc-600 transition" to="/apps">Apps</NuxtLink>
    </div>
    <div class="pt-14">
      <slot/>
    </div>
  </div>
</template>

<style scoped>
.router-link-active {
  @apply bg-zinc-50 text-neutral-900
}
</style>