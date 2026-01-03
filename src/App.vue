<script setup lang="ts">
import { ref } from "vue";
import MapView from "./components/Map.vue";
import { loadSkiAreas } from "./services/skiService";

const mapRef = ref<InstanceType<typeof MapView>>();

async function getSkiAreas() {
  const pistes = await loadSkiAreas(47.2682, 11.3923, 5000);
  mapRef.value?.addPisteMarkers(pistes);
}
</script>

<template>
  <div class="app-root">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="logo-container">
        <img src="/Logo_APP.png" alt="App Logo" class="logo" />
        <h1>Ski Explorer</h1>
      </div>
      <div class="controls">
        <button @click="getSkiAreas">Load Ski Areas</button>
      </div>
      <footer class="sidebar-footer">
        <p>© 2026 Ski Explorer</p>
      </footer>
    </aside>

    <!-- Main content -->
    <main class="main-content">
      <MapView ref="mapRef" class="map" />
    </main>
  </div>
</template>

<style scoped>
/* App root layout */
.app-root {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  background-color: var(--bg-color);
  color: var(--text-color);
}

/* Sidebar */
.sidebar {
  width: 300px;
  background-color: var(--sidebar-bg);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 2rem 1rem;
  box-shadow: 2px 0 6px rgba(0, 0, 0, 0.1);
}

.logo-container {
  text-align: center;
}

.logo {
  width: 100px;
  margin-bottom: 1rem;
  transition: filter 0.5s;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.controls {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-top: 2rem;
}

button {
  padding: 0.8rem 1.2rem;
  border-radius: 8px;
  border: 1px solid transparent;
  font-size: 1rem;
  font-weight: 500;
  background-color: #ffffff;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
  transition: all 0.3s;
}
button:hover {
  border-color: #396cd8;
}
button:active {
  background-color: #e8e8e8;
}

.sidebar-footer {
  text-align: center;
  font-size: 0.85rem;
  color: #888;
}

/* Main content (map) */
.main-content {
  flex: 1;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: var(--main-bg);
}

.map {
  width: 95%;
  height: 95%;
  border-radius: 12px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
}

/* Light/Dark Mode Variables */
:root {
  --bg-color: #f6f6f6;
  --text-color: #0f0f0f;
  --sidebar-bg: #ffffff;
  --main-bg: #eaeaea;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg-color: #2f2f2f;
    --text-color: #f6f6f6;
    --sidebar-bg: #1f1f1f;
    --main-bg: #333;
  }
}
</style>
