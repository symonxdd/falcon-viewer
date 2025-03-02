<template>
  <div class="image-gallery">
    <RecycleScroller class="scroller" :items="images" :item-size="216" :grid-items="dynamicGridItems"
      key-field="fileName">
      <template #default="{ item }">
        <div class="item" @click="imageClick(item)">
          <img v-if="item.miniature" :src="item.miniature" class="square-image" />
        </div>
      </template>
    </RecycleScroller>
  </div>
</template>

<script setup>
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
import { RecycleScroller } from "vue-virtual-scroller";
import { ref, computed, onMounted, onBeforeUnmount } from "vue";

// Props to receive the images array
const props = defineProps({
  images: {
    type: Array,
    required: true,
  },
});

// Emit the image click event
const emit = defineEmits(["image-click"]);

function imageClick(item) {
  emit("image-click", item);
}

// Grid item settings
const dynamicGridItems = ref(5); // Default value
const itemWidth = 202;
const margin = 16; // Combined left + right margin 

// Compute the total row width dynamically
const computedWrapperWidth = computed(() => {
  return `${dynamicGridItems.value * itemWidth + (dynamicGridItems.value - 1) * margin}px`;
});

// Function to recalculate grid items
function calculateGridItems() {
  const containerWidth = document.querySelector(".scroller").clientWidth;
  dynamicGridItems.value = Math.floor(containerWidth / (itemWidth + margin));
}

// Recalculate grid items on window resize
onMounted(() => {
  calculateGridItems();
  window.addEventListener("resize", calculateGridItems);
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", calculateGridItems);
});
</script>

<style scoped>
.image-gallery {
  margin-top: 3.5rem;
  user-select: none;
}

.item {
  width: 200px;
  height: 200px;
  border-radius: 8px;
  margin: 8px;
}

.square-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 8px;
}

/* "deep" selector in a scoped style, affecting child components (RecycleScroller here) */
/* source: (https://vuejs.org/api/sfc-css-features.html) */
:deep(.vue-recycle-scroller__item-wrapper) {
  width: v-bind(computedWrapperWidth) !important;
  margin: 0 auto;
}
</style>
