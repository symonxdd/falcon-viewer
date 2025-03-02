<template>
  <button class="close-button" :class="{ 'hidden-elements': isDragging }" @click="closeModal">
    <i class="bi bi-x-circle"></i>
  </button>

  <div class="overlay" @click="closeModal" @wheel="onScroll">
    <div class="modal">
      <img class="full-image" ref="imgElement" :src="image.fullImage" @click.stop :style="imageStyle"
        @mousedown="startDrag" @dblclick="handleDoubleClick" @load="updateImageDimensionsText" />

      <div class="image-information">
        <div class="image-buttons-group" @click.stop>
          <div class="image-actions" :class="{ 'hidden-elements': isDragging }">
            <button @click="toggleLike">
              <i class="bi bi-heart-fill" :style="{ color: heartColor }"></i>
            </button>
            <button @click="deleteImage">
              <i class="bi bi-trash2"></i>
            </button>
          </div>

          <div class="image-navigation" :class="{ 'hidden-elements': isDragging }">
            <button @click="previousImage">
              <i class="bi bi-arrow-left"></i>
            </button>
            <button @click="nextImage">
              <i class="bi bi-arrow-right"></i>
            </button>
          </div>
        </div>

        <div class="image-filename" @click.stop :class="{ 'hidden-elements': isDragging }">
          {{ image.fileName }}
        </div>
      </div>

    </div>
  </div>

  <div class="bottom-left-container">
    <div v-if="imageDimensionsText" class="image-dimensions" :class="{ 'hidden-elements': isDragging }">
      {{ imageDimensionsText }}
    </div>

    <button class="mode-toggle" :class="{ 'hidden-elements': isDragging }" @click="toggleMode">
      <i class="bi bi-arrows-vertical" v-if="isScrollMode"></i>
      <i class="bi bi-zoom-in" v-if="!isScrollMode"></i>
    </button>
  </div>

  <ConfirmDialog v-if="showConfirmDialog" message="Are you sure you want to delete this image?" @confirm="confirmDelete"
    @cancel="showConfirmDialog = false" />
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import ConfirmDialog from './ConfirmDialog.vue';
import { invoke } from "@tauri-apps/api/core";
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
const emit = defineEmits(["close", "changeImage"]);

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
const isLiked = ref(props.image.isLiked);

// Needed to reset when double-clicked
const initialZoom = ref(1);
const initialTranslateX = ref(0);
const initialTranslateY = ref(0);

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

const toggleLike = async () => {
  isLiked.value = !isLiked.value; // Instantly update UI

  try {
    const newLikeState = await invoke("toggle_like", {
      image_path: props.image.fullImagePath,
    });
    props.image.isLiked = newLikeState;
    console.log(`Image ${props.image.fullImagePath} has changed it's like state to ${newLikeState}`);
  } catch (error) {
    console.error("Failed to toggle like:", error);
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

const heartColor = computed(() => (isLiked.value ? '#D63031' : '#a8a8a8'));
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

  document.addEventListener("mousemove", onDrag);
  document.addEventListener("mouseup", stopDrag);
  document.addEventListener("mouseleave", stopDrag);
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

  document.removeEventListener("mousemove", onDrag);
  document.removeEventListener("mouseup", stopDrag);
  document.removeEventListener("mouseleave", stopDrag);
};

const closeModal = (event) => {
  if (event.target.closest(".modal")) return;
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

const handleMouseBackButton = (event) => {
  if (event.button === 3) {
    emit("close");
  }
};

const previousImage = () => {
  emit("changeImage", "prev");
};

const nextImage = () => {
  emit("changeImage", "next");
};

// Register and cleanup event listeners
onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
  document.addEventListener("mousedown", handleMouseBackButton);

  // Store initial values when the component is mounted
  initialZoom.value = zoomFactor.value;
  initialTranslateX.value = translateX.value;
  initialTranslateY.value = translateY.value;
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeydown);
  document.removeEventListener("mousedown", handleMouseBackButton);
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
  z-index: 2000;
  backdrop-filter: blur(50px);
  -webkit-backdrop-filter: blur(50px);
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
  box-shadow: 0px 12px 24px rgba(0, 0, 0, 0.8);
  /* ✅ Allows interactions with the image */
}

.close-button {
  position: fixed;
  top: 10px;
  right: 10px;
  color: #acacac;
  border-radius: 15px;
  font-size: 14px;
  z-index: 2001;
  cursor: pointer;

  padding: 4px 10px;
  border-radius: 5px;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.6s ease;
}

.bottom-left-container {
  position: fixed;
  bottom: 10px;
  left: 10px;
  display: flex;
  align-items: center;
  z-index: 2001;
}

.image-dimensions {
  color: #acacac;
  padding: 5px 10px;
  border-radius: 5px;
  font-size: 14px;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.6s ease;
}

.mode-toggle {
  margin-left: 8px;
  padding: 4px 10px;
  border-radius: 5px;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.6s ease;
}

.hidden-elements {
  opacity: 0;
  pointer-events: none;
}

.image-information {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.image-buttons-group {
  display: flex;
  gap: 10px;
  /* Space between action buttons and navigation buttons */
  align-items: center;
}

.image-actions,
.image-navigation {
  display: flex;
  gap: 15px;
  /* Keeps space between buttons inside each group */
  background-color: rgba(15, 15, 15, 0.7);
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.5);
  border: 1px solid #272727;
  padding: 3px 14px;
  border-radius: 20px;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.6s ease;
  pointer-events: auto;
}

.image-navigation {
  border-radius: 20px;
  padding: 4.5px 14px;
}

.image-filename {
  background-color: rgba(15, 15, 15, 0.7);
  box-shadow: 0px 12px 24px rgba(0, 0, 0, 0.5);
  border: 1px solid #272727;
  color: #e4e4e4;
  padding: 8px 20px;
  border-radius: 10px;
  /* 5px */
  user-select: text;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  transition: all 0.6s ease;
  pointer-events: auto;
}

.bi {
  cursor: pointer;
  color: #a8a8a8;
}

.bi.bi-x-circle {
  font-size: 1rem;
}

.bi.bi-heart-fill,
.bi.bi-trash2,
.bi.bi-arrow-left,
.bi.bi-arrow-right {
  font-size: 1.3rem;
  transition: color 0.3s ease;
}

.bi.bi-trash2 {
  font-size: 1.4rem;
}
</style>
