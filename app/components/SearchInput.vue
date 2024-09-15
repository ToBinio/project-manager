<script setup lang="ts">
let text = defineModel("text")

let inputRef = useTemplateRef("input")

onKeyStroke("f", event => {
  if (!event.ctrlKey) return

  inputRef.value.focus()
})
</script>

<template>
  <div class="input h-7 bg-zinc-700 m-2 rounded-md w-max flex items-center z-10">
    <Icon class="mx-1.5" name="ph:magnifying-glass-bold" size="18"/>
    <input ref="input" class="h-full bg-transparent !outline-none" type="text" v-model="text">
  </div>
</template>

<style scoped>
.input {
  &:has(input:focus-visible) {
    @apply bg-zinc-600
  }
}
</style>