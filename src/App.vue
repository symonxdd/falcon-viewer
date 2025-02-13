<template>
  <div class="min-h-screen flex flex-col items-start justify-start bg-[#121212] text-[#e0e0e0] px-4">
    <!-- Folder Selection Button & Path -->
    <div class="flex items-center w-full mt-4">
      <button @click="selectFolder" class="px-6 py-3 bg-[#222] hover:bg-[#333] text-[#e0e0e0] rounded">
        Select Image Folder
      </button>
      <div v-if="selectedFolderPath" class="ml-4 text-[#b0b0b0] flex items-center">
        <p>{{ selectedFolderPath }} <span class="text-[#888]">({{ images.length }} items)</span></p>
      </div>
    </div>

    <ImageGallery v-if="images.length" :images="images" @image-click="showFullImage" />
    <FullImageView v-if="showFullImageView" :image="selectedImage" @close="closeFullImageView" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import ImageGallery from "./components/ImageGallery.vue";
import FullImageView from './components/FullImageView.vue';

// Reactive (state) variables
const images = ref([]);
const selectedFolderPath = ref("");
const showFullImageView = ref(false);
const selectedImage = ref(null);

async function selectFolder() {
  const selected = await open({ directory: true });
  if (selected) {
    selectedFolderPath.value = selected;

    // Fetch images and their thumbnail paths from Rust
    const rawImages = await invoke("get_images", { folder_path: selected });

    // Convert paths & store in the correct format
    images.value = rawImages.map(([fullPath, thumbPath]) => ({
      id: fullPath,
      full: convertFileSrc(fullPath),
      thumbnail: thumbPath ? convertFileSrc(thumbPath) : null,
      loaded: false,
    }));
  }
}

// Show the full image in the modal
function showFullImage(image) {
  selectedImage.value = image;
  showFullImageView.value = true;
}

// Close the full image view modal
function closeFullImageView() {
  showFullImageView.value = false;
  selectedImage.value = null;
}
</script>

<style scoped></style>
