<script setup>
import { theme, setTheme } from "./composables/useTheme";

function toggleTheme() {
  if (theme.value === "light") setTheme("dark");
  else setTheme("light");
}
</script>

<template>
  <div
    class="min-h-screen flex flex-col items-center justify-center bg-gray-100 dark:bg-gray-900 text-black dark:text-white">
    <h1 class="text-3xl font-bold">Tauri Image Viewer</h1>
    <button @click="toggleTheme" class="mt-4 px-4 py-2 bg-blue-500 text-white rounded">
      Toggle Theme
    </button>
  </div>
</template>