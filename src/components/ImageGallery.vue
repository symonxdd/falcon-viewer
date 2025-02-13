<template>
  <div class="w-full mt-6">
    <RecycleScroller page-mode class="scroller" :items="images" :item-size="200" :grid-items="4"
      :item-secondary-size="200" key-field="id">
      <template #default="{ item }">
        <div class="item" @click="imageClick(item)">
          <!-- Show Thumbnail -->
          <img v-if="item.thumbnail" :src="item.thumbnail" class="square-image" />
        </div>
      </template>
    </RecycleScroller>
  </div>
</template>

<script setup>
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
import { RecycleScroller } from "vue-virtual-scroller";

// Props to receive the images array
const props = defineProps({
  images: {
    type: Array,
    required: true
  }
});

// Emit the image click event
const emit = defineEmits(['image-click']);

function imageClick(item) {
  emit('image-click', item);
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
  position: relative;
  width: 100%;
  height: 200px;
}

.square-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 8px;
}
</style>
