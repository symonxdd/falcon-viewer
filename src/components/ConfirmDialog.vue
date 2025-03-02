<template>
  <div class="dialog-overlay">
    <div class="dialog">
      <p class="dialog-message">{{ message }}</p>
      <div class="dialog-actions">
        <button class="confirm-btn" @click="$emit('confirm')">Yes</button>
        <button class="cancel-btn" @click="$emit('cancel')">No</button>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({ message: String });
</script>

<style scoped>
/* Fullscreen overlay */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.6);
  /* Darker overlay for more contrast */
  backdrop-filter: blur(10px);
  /* Stronger glass effect */
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 2000;
  pointer-events: auto;
}

/* Modal box */
.dialog {
  background: rgba(28, 28, 30, 0.8);
  /* Dark semi-transparent background */
  border: 1px solid rgba(255, 255, 255, 0.2);
  /* Subtle border */
  padding: 32px 24px;
  border-radius: 14px;
  width: 380px;
  /* Slightly wider for better layout */
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
  /* More subtle shadow */
  text-align: center;
  transition: all 0.3s ease-in-out;
}

/* Message text */
.dialog-message {
  color: #e4e4e4;
  font-size: 18px;
  /* Slightly larger for better readability */
  margin-bottom: 24px;
  font-weight: 500;
  line-height: 1.5;
  /* Better line spacing */
}

/* Buttons container */
.dialog-actions {
  display: flex;
  justify-content: space-between;
  margin-top: 20px;
}

/* Modern buttons */
button {
  flex: 1;
  padding: 12px 20px;
  /* Slightly larger padding */
  border-radius: 8px;
  /* Softer corners */
  font-size: 16px;
  /* Larger font size for readability */
  font-weight: bold;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  outline: none;
  border: 1px solid transparent;
  /* Transparent border initially */
  background: transparent;
  /* Transparent background */
  color: white;
  /* White text color */
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  /* Subtle shadow for buttons */
}

/* Confirm button */
.confirm-btn {
  /* border-color: #1156be; */
  background-color: #1156be;
  /* border-color: #636363; */
}

.confirm-btn:hover {
  background: rgba(76, 175, 80, 0.1);
  /* Light green hover effect */
  color: #4caf50;
  /* Text color changes to green */
  border-color: #388e3c;
  /* Darker green border on hover */
  transform: scale(1.05);
  /* Slight scale on hover */
}

/* Cancel button */
.cancel-btn {
  border-color: #636363;
  margin-left: 12px;
}

.cancel-btn:hover {
  background: rgba(229, 57, 53, 0.1);
  /* Light red hover effect */
  color: #e53935;
  /* Text color changes to red */
  border-color: #c62828;
  /* Darker red border on hover */
  transform: scale(1.05);
  /* Slight scale on hover */
}
</style>
