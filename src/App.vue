<template>
  <div class="min-h-screen flex flex-col bg-[#121212] text-[#e0e0e0] px-4">
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
import { ref, onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import ImageGallery from "./components/ImageGallery.vue";
import FullImageView from './components/FullImageView.vue';

// Reactive (state) variables
const images = ref([]);
const selectedFolderPath = ref("");
const showFullImageView = ref(false);
const selectedImage = ref(null);

let unlisten;

onMounted(async () => {
  // 🎧 Listen for "image-processed" events
  unlisten = await listen("image-processed", (event) => {
    const { fileName, imagePath, miniaturePath } = event.payload;

    images.value = [
      ...images.value,
      {
        fileName: fileName.replace(/\.[^/.]+$/, ""),
        fullImage: convertFileSrc(imagePath),
        fullImagePath: imagePath,
        miniature: convertFileSrc(miniaturePath)
      }
    ];
  });
});

onUnmounted(() => {
  if (unlisten) {
    // 🛑 Stop listening when component unmounts
    unlisten();
  }
});

async function selectFolder() {
  const selected = await open({ directory: true });
  if (selected) {
    selectedFolderPath.value = selected;

    // 🚀 Start image processing in Rust (no return value)
    const rawImages = await invoke("process_images", { folder_path: selected });
  }
}

// Show the full image in the modal
function showFullImage(item) {
  selectedImage.value = item;
  showFullImageView.value = true;
}

// Close the full image view modal
function closeFullImageView() {
  showFullImageView.value = false;
  selectedImage.value = null;
}
</script>

<style scoped></style>
