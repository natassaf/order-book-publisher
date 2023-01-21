import { createApp } from 'vue'
import App from './App.vue'
import Vue3EasyDataTable from 'vue3-easy-data-table';
import 'vue3-easy-data-table/dist/style.css';

import OrderBookVisualizer from "./components/OrderBookVisualizer.vue";
const app = createApp(App);

app.component('EasyDataTable', Vue3EasyDataTable);

app.component("order-book-visualizer", OrderBookVisualizer);
app.mount('#app');
