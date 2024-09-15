<script setup lang="ts">
import AppList from "~/components/apps/AppList.vue";

let apps = await useApps()

let searchText = ref("")

let filteredApps = computed(() => {
  return apps.value
      .sort((a, b) => a.name.localeCompare(b.name))
      .filter(app => app.name.toUpperCase().includes(searchText.value.toUpperCase()))
})

let inputRef = useTemplateRef("input")

onKeyStroke("f", event => {
  if (!event.ctrlKey) return

  inputRef.value.focus()
})

</script>

<template>
  <div class="bg-zinc-900 fixed w-full">
    <div class="input h-7 bg-zinc-700 m-2 rounded-md w-max flex items-center z-10">
      <Icon class="mx-1.5" name="ph:magnifying-glass-bold" size="18"/>
      <input ref="input" class="h-full bg-transparent !outline-none" type="text" v-model="searchText">
    </div>
  </div>

  <div class="mt-9">
    <AppList :apps="filteredApps"/>
  </div>
</template>

<style scoped>
.input {
  &:has(input:focus-visible) {
    @apply bg-zinc-600
  }
}
</style>