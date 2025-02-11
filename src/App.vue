<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-[#121212] text-[#e0e0e0]">
    <h1 class="text-3xl font-bold">Tauri Image Viewer</h1>

    <!-- Folder Selection Button -->
    <button @click="selectFolder" class="mt-4 px-4 py-2 bg-[#222] hover:bg-[#333] text-[#e0e0e0] rounded">
      Select Image Folder
    </button>

    <!-- Display selected folder path -->
    <div v-if="folderPath" class="mt-4 text-[#b0b0b0]">
      <p>Selected Folder: {{ folderPath }}</p>
    </div>

    <div v-if="images.length" class="grid grid-cols-4 gap-4 mt-6 p-4">
      <div v-for="image in images" :key="image" class="relative group">
        <!-- <img :src="image" class="w-48 h-48 object-cover rounded shadow-md" /> -->

        <img :src="image" class="w-48 h-48 object-cover rounded shadow-md" />

        <div class="absolute bottom-0 w-full bg-black bg-opacity-50 text-white text-sm p-1 truncate">
          {{ image.split('/').pop() }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";

const folderPath = ref("");
const images = ref([]);

// Folder selection function
async function selectFolder() {
  const selected = await open({ directory: true });
  if (selected) {
    folderPath.value = selected;

    // Fetch image paths from Rust
    const rawImages = await invoke("get_images", { folder_path: selected });

    // ✅ Convert paths to be compatible with WebView
    images.value = rawImages.map(img => convertFileSrc(img));
  }
}
</script>
