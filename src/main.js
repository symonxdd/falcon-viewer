import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";
import Toast from "vue-toastification";
import "vue-toastification/dist/index.css";
import 'bootstrap-icons/font/bootstrap-icons.css';

const app = createApp(App);

app.use(Toast, {
  timeout: 2000, // ⏳ Make all toasts disappear in 2 seconds
  pauseOnHover: false, // Don't pause when hovered
});

app.mount("#app");