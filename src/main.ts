import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import { router, i18nModule } from "@/modules";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18nModule);

app.mount("#app");

import "@/assets/index.css";
