import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css"; // ✅ Import Tailwind CSS!

import Toast from "vue-toastification";
import "vue-toastification/dist/index.css";

// ✅ Import Vue Unicons
import Unicon from "vue-unicons";
import {
  uniDirection,
  uniSearchPlus,
  uniHeart,
  uniTrashAlt,
  uniTimesCircle
} from "vue-unicons/dist/icons";

Unicon.add([
  uniDirection,
  uniSearchPlus,
  uniHeart,
  uniTrashAlt,
  uniTimesCircle]);

const app = createApp(App);
app.use(Unicon);
app.use(Toast);
app.mount("#app");
