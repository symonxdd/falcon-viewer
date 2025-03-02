<template>
  <div class="welcome-screen">
    <!-- Animated Gradient Title -->
    <h1 class="title">Falkon Viewer</h1>

    <!-- Subtitle -->
    <p class="subtitle">Effortless image browsing</p>

    <!-- Select Folder Button -->
    <button @click="$emit('select-folder')" class="select-folder-button">
      Select folder
    </button>
  </div>
</template>

<script setup>
</script>

<style scoped>
.welcome-screen {
  text-align: center;
  position: absolute;
  top: 46%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #e0e0e0;
}

/* Smooth Gradient Animation for Title */
.title {
  font-size: 2.5rem;
  font-weight: bold;
  background: linear-gradient(90deg, #6a11cb, #2575fc, #6a11cb);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-size: 300% 300%;
  opacity: 0;
  animation: fadeIn 1.2s ease-out forwards, gradientFlow 10s infinite linear;
}

/* Subtitle */
.subtitle {
  font-size: 1rem;
  color: #aaaaaa;
  margin-top: 0.5rem;
  opacity: 0;
  animation: fadeIn 1.5s ease-out forwards;
}

.select-folder-button {
  margin-top: 1.5rem;
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  font-weight: bold;
  color: #e0e0e0;
  background-color: #222;
  /* Default dark color */
  border-radius: 50px;
  /* Fully Rounded */
  border: 2px solid transparent;
  /* Invisible border initially */
  position: relative;
  overflow: hidden;
  transition: transform 0.3s ease-in-out;
}

/* Hover Effect */
.select-folder-button:hover {
  transform: scale(1.02);
}

/* Add pseudo-element for the gradient border */
.select-folder-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 50px;
  background: linear-gradient(90deg, #ff3cac, #6a11cb, #2575fc);
  background-size: 400% 400%;
  z-index: -1;
  animation: gradientMovement 7s linear infinite;
}

/* Continuous Gradient Animation */
@keyframes gradientMovement {
  0% {
    background-position: 0% 0%;
  }

  25% {
    background-position: 100% 0%;
  }

  50% {
    background-position: 100% 100%;
  }

  75% {
    background-position: 0% 100%;
  }

  100% {
    background-position: 0% 0%;
  }
}

@keyframes gradientFlow {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

/* 🔥 Keyframe Animation for Circular Motion */
@keyframes circularBorder {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
