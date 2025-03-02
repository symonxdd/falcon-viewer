<template>
  <nav class="menu-bar">
    <div class="menu-item" ref="menuRef">
      <button class="menu-button" @click="toggleMenu">File</button>
      <div v-if="isMenuOpen" class="submenu" @mouseleave="closeMenu">
        <button class="submenu-item" @click="onSelectFolderClick">
          Select folder...
        </button>
      </div>
    </div>
  </nav>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";

const isMenuOpen = ref(false);
const menuRef = ref(null);
const emit = defineEmits(["select-folder"]); // Define event

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value;
};

const closeMenu = () => {
  isMenuOpen.value = false;
};

const onSelectFolderClick = () => {
  console.log("clicked Select folder item");
  emit("select-folder"); // Emit event to parent
  closeMenu();
};

const handleClickOutside = (event) => {
  if (menuRef.value && !menuRef.value.contains(event.target)) {
    closeMenu();
  }
};

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
});
</script>

<style scoped>
.menu-bar {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  z-index: 10;
  user-select: none;
  background-color: #121212;
  border-bottom: 1px solid #181818;
}

.menu-item {
  position: relative;
}

.menu-button {
  background: none;
  border: none;
  padding: 3px 12px;
  cursor: pointer;
  font-size: 14px;
  color: #ddd;
}

.menu-button:hover {
  background: #2b2b2b;
}

.submenu {
  position: absolute;
  top: 100%;
  left: 0;
  background: #252525;
  /* border: 1px solid #444; */
  box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.5);
  padding: 4px 0;
  min-width: 150px;
}

.submenu-item {
  display: block;
  width: 100%;
  padding: 6px 12px;
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  font-size: 14px;
  color: #bbb;
}

.submenu-item:hover {
  background: #444;
  color: #fff;
}
</style>
