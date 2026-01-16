import { createApp } from "vue";
import App from "./App.vue";
import "./assets/css/tailwind.css";

import "./leafletFix";
import "leaflet/dist/leaflet.css";

createApp(App).mount("#app");
