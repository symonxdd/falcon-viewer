import { ref, onMounted } from "vue";

const theme = ref(localStorage.getItem("theme") || "auto");

function setTheme(mode) {
  theme.value = mode;
  if (mode === "dark") {
    document.documentElement.classList.add("dark");
    localStorage.setItem("theme", "dark");
  } else if (mode === "light") {
    document.documentElement.classList.remove("dark");
    localStorage.setItem("theme", "light");
  } else {
    // Auto mode (system-based)
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    document.documentElement.classList.toggle("dark", prefersDark);
    localStorage.setItem("theme", "auto");
  }
}

onMounted(() => setTheme(theme.value));

export { theme, setTheme };