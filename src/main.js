import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css"; // ✅ Import Tailwind CSS!
import Unicon from 'vue-unicons'
import { uniDirection, uniSearchPlus } from 'vue-unicons/dist/icons'

Unicon.add([uniDirection, uniSearchPlus])

createApp(App).use(Unicon).mount("#app");
