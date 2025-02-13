<template>
  <button class="mode-toggle" @click="toggleMode">
    <unicon v-if="isScrollMode" name="direction" :fill="'#acacac'" width="19" />
    <unicon v-if="!isScrollMode" name="search-plus" :fill="'#acacac'" width="19" />
  </button>

  <div class="overlay" @click="closeModal" @wheel="onScroll">
    <div class="modal">
      <img ref="imgElement" :src="image.full" class="full-image" @click.stop :style="imageStyle" @mousedown="startDrag"
        @mousemove="onDrag" @mouseup="stopDrag" @mouseleave="stopDrag" @dblclick="handleDoubleClick"
        @load="updateImageDimensionsText" />
    </div>
  </div>

  <div v-if="imageDimensionsText" class="image-dimensions">
    {{ imageDimensionsText }}
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";

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

// Needed to reset when double-clicked
const initialZoom = ref(1);
const initialTranslateX = ref(0);
const initialTranslateY = ref(0);

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
    zoomFactor.value = Math.max(0.5, Math.min(zoomFactor.value, maxZoom));
  }
};

// const onScroll = (event) => {
//   // Check if the CTRL key is pressed
//   if (event.ctrlKey) {
//     const scrollAmount = event.deltaY > 0 ? -VERTICAL_SCROLL_SPEED : VERTICAL_SCROLL_SPEED; // Scroll down = move down, Scroll up = move up
//     translateY.value += scrollAmount;
//   } else {
//     // const zoomDelta = event.deltaY > 0 ? -ZOOM_SPEED : ZOOM_SPEED; // Zoom in or out
//     // zoomFactor.value += zoomDelta; // Apply zoom change directly without sensitivity based on zoom level
//     // zoomFactor.value = Math.max(0.5, Math.min(zoomFactor.value, 3)); // Clamp zoom level (for example between 0.5 and 3)

//     // Prevent the default scroll behavior (page scroll)
//     event.preventDefault();

//     // Zooming in (scroll up) or zooming out (scroll down)
//     const zoomDelta = event.deltaY < 0 ? ZOOM_SPEED : -ZOOM_SPEED; // Scroll up = zoom in, scroll down = zoom out

//     // Apply the zoom change
//     zoomFactor.value += zoomDelta;

//     // Clamp zoom to ensure it doesn't go below 0.5x or exceed a reasonable max zoom level
//     // const maxZoom = 5;  // Limit zoom max to 5x
//     // zoomFactor.value = Math.max(0.5, Math.min(zoomFactor.value, maxZoom)); // Prevent zooming too much
//   }
// };

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
  if (event.target === event.currentTarget) {
    emit("close");
  }
};

// Close modal on Escape key
const handleKeydown = (event) => {
  if (event.key === "Escape") {
    emit("close");
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

.image-dimensions {
  position: fixed;
  top: 10px;
  right: 10px;
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
  padding: 0px 20px;
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
</style>
