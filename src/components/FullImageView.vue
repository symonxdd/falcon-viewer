<template>
  <button class="mode-toggle" @click="toggleMode">
    <unicon v-if="isScrollMode" name="direction" :fill="'#acacac'" width="19" />
    <unicon v-if="!isScrollMode" name="search-plus" :fill="'#acacac'" width="19" />
  </button>

  <button class="close-button" @click="closeModal">
    <unicon name="times-circle" fill="#acacac" width="21" />
  </button>

  <div class="overlay" @click="closeModal" @wheel="onScroll">

    <div class="modal">
      <img ref="imgElement" :src="image.fullImage" class="full-image" @click.stop :style="imageStyle"
        @mousedown="startDrag" @mousemove="onDrag" @mouseup="stopDrag" @mouseleave="stopDrag"
        @dblclick="handleDoubleClick" @load="updateImageDimensionsText" />

      <div class="image-actions" @click.stop>
        <button class="" @click="toggleLike">
          <unicon name="heart" :fill="isLiked ? 'red' : '#acacac'" width="21" />
        </button>
        <button class="" @click="deleteImage">
          <unicon name="trash-alt" fill="#acacac" width="21" />
        </button>
      </div>

      <div class="image-filename" @click.stop>
        {{ image.fileName }}
      </div>
    </div>
  </div>

  <div v-if="imageDimensionsText" class="image-dimensions">
    {{ imageDimensionsText }}
  </div>

  <ConfirmDialog v-if="showConfirmDialog" message="Are you sure you want to delete this image?" @confirm="confirmDelete"
    @cancel="showConfirmDialog = false" />
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import ConfirmDialog from './ConfirmDialog.vue';
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { useToast } from 'vue-toastification';
const toast = useToast();

// Props
const props = defineProps({
  image: {
    type: Object,
    required: true,
  },
});

// Emit event for closing modal
const emit = defineEmits(["close"]);

// Reactive properties
const imgElement = ref(null);
const zoomFactor = ref(1);
const translateX = ref(0);
const translateY = ref(0);
const isDragging = ref(false);
const startX = ref(0);
const startY = ref(0);
const imageDimensionsText = ref(""); // Store image dimensions
const isScrollMode = ref(false); // Default to scroll mode
const showConfirmDialog = ref(false);

// Needed to reset when double-clicked
const initialZoom = ref(1);
const initialTranslateX = ref(0);
const initialTranslateY = ref(0);
const isLiked = ref(false);

const toggleLike = () => {
  isLiked.value = !isLiked.value;
  console.log("Heart Clicked!");
};

const confirmDelete = async () => {
  showConfirmDialog.value = false; // Close the dialog

  try {
    // Invoke the Rust method to delete the image
    await invoke("delete_image", { image_path: props.image.fullImagePath });

    // After deleting, show a toast notification
    toast.success("Image deleted successfully!");
    emit("close"); // Close the modal after successful deletion
  } catch (error) {
    console.error("Error deleting image:", error);
    toast.error("Failed to delete the image. Please try again.");
  }
};

const deleteImage = async () => {
  showConfirmDialog.value = true;
};

// Toggle between zoom and scroll modes
const toggleMode = () => {
  isScrollMode.value = !isScrollMode.value;
};

// Function to update image dimensions
const updateImageDimensionsText = () => {
  if (imgElement.value) {
    imageDimensionsText.value = `${imgElement.value.naturalWidth} x ${imgElement.value.naturalHeight}`;
  }
};

// Zooming factor (change this to adjust zoom speed)
const ZOOM_SPEED = 0.2; // 30% per scroll step
const VERTICAL_SCROLL_SPEED = 35; // Movement speed

// Compute dynamic image styles
const imageStyle = computed(() => ({
  transform: `scale(${zoomFactor.value}) translate(${translateX.value}px, ${translateY.value}px)`,
  transition: isDragging.value ? "none" : "transform 0.2s ease",
  maxWidth: "calc(100vw - 80px)", // 40px margin on both sides
  maxHeight: "calc(100vh - 80px)", // 40px margin on top/bottom
  objectFit: "contain",
  cursor: isDragging.value ? "move" : "pointer",
}));

const handleDoubleClick = () => {
  // If image is at default zoom and position, apply the zoom-in logic
  if ((zoomFactor.value === initialZoom.value)
    && (translateX.value === initialTranslateX.value)
    && (translateY.value === initialTranslateY.value)) {

    const { naturalHeight } = imgElement.value;

    // Calculate zoom factor based on image dimensions
    let zoomInFactor = 1.4; // Default zoom-in factor for standard images

    // For very wide or tall images, adjust the zoom factor dynamically
    if (naturalHeight > 1080) {
      // If image is wide or tall, zoom more
      zoomInFactor = 7;
    } else if (naturalHeight > 1000) {
      // Moderate zoom factor for medium-sized images
      zoomInFactor = 1.75;
    }

    // Apply zoom
    zoomFactor.value *= zoomInFactor;
  } else {
    zoomFactor.value = initialZoom.value;
    translateX.value = initialTranslateX.value;
    translateY.value = initialTranslateY.value;
  }
};

const onScroll = (event) => {
  event.preventDefault(); // Prevent default scroll behavior

  if (isScrollMode.value) {
    // Scroll Mode: Moves the image vertically
    const scrollAmount = event.deltaY > 0 ? -VERTICAL_SCROLL_SPEED : VERTICAL_SCROLL_SPEED;
    translateY.value += scrollAmount;
  } else {
    // Zoom Mode: Zooms in or out
    const zoomDelta = event.deltaY < 0 ? ZOOM_SPEED : -ZOOM_SPEED;
    zoomFactor.value += zoomDelta;

    // Clamp zoom values
    const maxZoom = 5;
    zoomFactor.value = Math.max(0.5, Math.min(zoomFactor.value, 500));
  }
};

// Dragging the Image
const startDrag = (event) => {
  event.preventDefault(); // 😌 Prevent default behavior to allow dragging

  isDragging.value = true;

  // Calculate initial offset based on zoom level
  startX.value = event.clientX - translateX.value * zoomFactor.value;
  startY.value = event.clientY - translateY.value * zoomFactor.value;
};

const onDrag = (event) => {
  if (!isDragging.value) return;

  // Move image based on current mouse position and zoom level
  const deltaX = (event.clientX - startX.value) / zoomFactor.value;
  const deltaY = (event.clientY - startY.value) / zoomFactor.value;

  // Apply calculated movement
  translateX.value = deltaX;
  translateY.value = deltaY;
};

const stopDrag = () => {
  isDragging.value = false;
};

const closeModal = (event) => {
  emit("close");
};

// Close modal on Escape key
const handleKeydown = (event) => {
  if (event.key === "Escape") {
    emit("close");
  }
  if (event.key === "Delete") {
    showConfirmDialog.value = true;
  }
};

// Register and cleanup event listeners
onMounted(() => {
  window.addEventListener("keydown", handleKeydown);

  // Store initial values when the component is mounted
  initialZoom.value = zoomFactor.value;
  initialTranslateX.value = translateX.value;
  initialTranslateY.value = translateY.value;
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeydown);
});
</script>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal {
  display: flex;
  justify-content: center;
  align-items: center;
  pointer-events: none;
  /* ✅ Ensures modal itself doesn't block clicks */
}

.full-image {
  user-select: none;
  pointer-events: auto;
  /* ✅ Allows interactions with the image */
}

.close-button {
  position: fixed;
  top: 10px;
  right: 10px;
  background-color: rgba(0, 0, 0, 0.6);
  color: #acacac;
  padding: 5px 10px;
  border-radius: 15px;
  font-family: Arial, sans-serif;
  font-size: 14px;
  z-index: 1000;
  cursor: pointer;

  /* Flexbox to center the icon */
  display: flex;
  align-items: center;
  justify-content: center;
}

.image-dimensions {
  position: fixed;
  bottom: 10px;
  left: 10px;
  background-color: rgba(0, 0, 0, 0.6);
  color: #acacac;
  padding: 5px 10px;
  border-radius: 5px;
  font-family: Arial, sans-serif;
  font-size: 14px;
  z-index: 1000;
}

.mode-toggle {
  position: fixed;
  top: 10px;
  left: 10px;
  background-color: rgba(0, 0, 0, 0.6);
  padding: 4px 8px;
  border-radius: 10px;
  cursor: pointer;
  z-index: 1000;
  display: flex;
  /* Use flex to center items */
  justify-content: center;
  /* Center the icon horizontally */
  align-items: center;
  /* Center the icon vertically */
}

.image-filename {
  position: absolute;
  top: 60px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(93, 65, 156, 0.95);
  border: 1px solid #2f2150;
  color: #e4e4e4;
  padding: 5px 10px;
  border-radius: 5px;
  font-size: 16px;
  user-select: text;
  pointer-events: auto;

  box-shadow: 0px 6px 12px rgba(0, 0, 0, 0.2),
    0px 12px 24px rgba(0, 0, 0, 0.15);
}

.image-actions {
  position: absolute;
  bottom: 50px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 15px;
  background: rgba(0, 0, 0, 0.6);
  padding: 1px 10px;
  border-radius: 5px;
  pointer-events: auto;
  /* ✅ Allow clicks */
}

.image-actions unicon {
  cursor: pointer;
  pointer-events: auto;
}
</style>
