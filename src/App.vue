<template>
  <div class="app-container">
    <CustomMenuBar @select-folder="selectFolder" />
    <WelcomeScreen v-if="!selectedFolderPath" @select-folder="selectFolder" />

    <div v-if="selectedFolderPath" class="folder-path">
      <p>{{ selectedFolderPath }} <span class="item-count">({{ images.length }} images)</span></p>
    </div>

    <ImageGallery v-if="images.length" :images="images" @image-click="showFullImage" />
    <FullImageView v-if="showFullImageView" :image="selectedImage" @close="closeFullImageView"
      @changeImage="changeImage" />
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import ImageGallery from "./components/ImageGallery.vue";
import FullImageView from './components/FullImageView.vue';
import CustomMenuBar from './components/CustomMenuBar.vue';
import WelcomeScreen from "./components/WelcomeScreen.vue";

// Reactive (state) variables
const images = ref([]);
const selectedFolderPath = ref("");
const showFullImageView = ref(false);
const selectedImage = ref(null);
const currentIndex = ref(0);

let unlisten;

onMounted(async () => {
  // 🎧 Listen for "image-processed" events
  unlisten = await listen("image-processed", (response) => {
    const { fileName, imagePath, miniaturePath, isLiked } = response.payload;

    images.value = [
      ...images.value,
      {
        fileName: fileName.replace(/\.[^/.]+$/, ""),
        fullImage: convertFileSrc(imagePath),
        miniature: convertFileSrc(miniaturePath),
        fullImagePath: imagePath,
        isLiked
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
    images.value = [];

    // 🚀 Start image processing in Rust (no return value)
    await invoke("process_images", { folder_path: selected });
  }
}

// Show the full image in the modal
function showFullImage(item) {
  selectedImage.value = item;
  currentIndex.value = images.value.findIndex(img => img.fullImagePath === item.fullImagePath);
  showFullImageView.value = true;
}

function changeImage(direction) {
  if (!images.value.length) return;

  if (direction === "next") {
    currentIndex.value = (currentIndex.value + 1) % images.value.length;
  } else if (direction === "prev") {
    currentIndex.value = (currentIndex.value - 1 + images.value.length) % images.value.length;
  }

  selectedImage.value = images.value[currentIndex.value];
}

// Close the full image view modal
function closeFullImageView() {
  showFullImageView.value = false;
  selectedImage.value = null;
}
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #121212;
  color: #e0e0e0;
  justify-content: flex-start;
  /* Keep other content at the top */
  position: relative;
  /* Necessary for positioning the folder path */
}

.folder-path {
  user-select: none;
  position: fixed;
  bottom: 0;
  color: #525252;
  font-size: 12px;
  z-index: 1000;
  font-weight: bold;
  background-color: #121212;
  width: 100vw;
  padding: 4px 8px;
  border-top: 1px solid #181818;
}

.item-count {
  color: #525252;
  font-size: 12px;
}
</style>
