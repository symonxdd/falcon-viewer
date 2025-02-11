<template>
  <div class="min-h-screen flex flex-col items-start justify-start bg-[#121212] text-[#e0e0e0] px-4">
    <!-- Container for the button and folder path aligned on the same line -->
    <div class="flex items-center w-full mt-4">
      <!-- Folder Selection Button -->
      <button @click="selectFolder" class="px-6 py-3 bg-[#222] hover:bg-[#333] text-[#e0e0e0] rounded">
        Select Image Folder
      </button>

      <!-- Display selected folder path with slight spacing and vertical centering -->
      <div v-if="folderPath" class="ml-4 text-[#b0b0b0] flex items-center">
        <p>{{ folderPath }}</p>
      </div>
    </div>

    <!-- Virtual Scroller for Images Grid -->
    <div v-if="images.length" class="w-full mt-6">
      <RecycleScroller class="scroller" :items="images" :item-size="200" :grid-items="4" :item-secondary-size="200">
        <template #default="{ item }">
          <div class="item">
            <img :src="item" class="square-image" />
          </div>
        </template>
      </RecycleScroller>
    </div>
  </div>
</template>

<script setup>
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'; // ✅ Very important...

import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { RecycleScroller } from "vue-virtual-scroller";
import { convertFileSrc } from "@tauri-apps/api/core";

// Reactive variables to hold the window size
const folderPath = ref("");
const images = ref([]);

// Folder selection function
async function selectFolder() {
  const selected = await open({ directory: true });
  if (selected) {
    folderPath.value = selected;

    // Fetch image paths from Rust
    const rawImages = await invoke("get_images", { folder_path: selected });

    // ✅ Convert local paths to the correct Tauri format
    images.value = rawImages.map(path => convertFileSrc(path));
  }
}
</script>

<style scoped>
.scroller {
  width: 100%;
  height: 100%;
}

.item {
  padding: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  /* background-color: #333; */
  position: relative;
  width: 100%;
  /* Ensure the item width takes the full space in the scroller */
  height: 200px;
  /* Fix item height to 200px */
}

.square-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  /* Ensures the image covers the square */
  border-radius: 8px;
}
</style>
