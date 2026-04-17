<script setup lang="ts">
import { ref } from "vue";
import MapView from "./components/Map.vue";
import { loadSkiAreas } from "./services/skiService";
import { Position } from "./types/position";
import { Slider } from "@/components/ui/slider";
import { useTauriDesktopGuards } from "./composables/useTauriDesktopGuards";
import { Spinner } from "@/components/ui/spinner";
import { loadStandort, Standort } from "./services/standortService";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";

useTauriDesktopGuards({
  blockFileDrop: true,
  blockContextMenu: true,
  blockContextMenuInDev: false,
});

const mapRef = ref<InstanceType<typeof MapView>>();
const posRef = ref<Position>({ lat: 52.52437, lon: 13.41053 });
const sliderRef = ref<number[] | undefined>([5]);
const errorRef = ref<string | null>(null);
const waitingForApi = ref(false);
const searchRef = ref<string>("");

async function getSkiAreas(pos: Position) {
  try {
    if (sliderRef.value) {
      waitingForApi.value = true;
      const pistes = await loadSkiAreas(pos, sliderRef.value[0] * 1000);
      waitingForApi.value = false;
      mapRef.value?.addPisteMarkers(pistes);
      errorRef.value = null;
    }
  } catch (error: unknown) {
    waitingForApi.value = false;
    errorRef.value =
      error instanceof Error ? error.message : "Unbekannter Fehler";
  }
}

async function getStandort() {
  try {
    if (searchRef.value && searchRef.value.length >= 0) {
      const standorte = await loadStandort(searchRef.value);
      mapRef.value?.setView(posRef.value);
    } else {
      let err = Error("Fehlende Eingabe");
      console.error(err);
      throw err;
    }
  } catch (error: unknown) {
    errorRef.value =
      error instanceof Error ? error.message : "Fehlende Eingabe";
  }
}
</script>

<template>
  <div class="app-root">
    <div class="data">Laurenz</div>
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-content">
        <div class="logo-section">
          <img src="/Logo_APP.png" alt="App Logo" class="logo" />
          <h1 class="app-title">Ski Explorer</h1>
        </div>

        <div class="control-group">
          <label class="control-label">Standort Eingabe</label>
          <input
            v-model="searchRef"
            type="text"
            placeholder="z.B. Berlin"
            class="text-input"
          />
          <button @click="getStandort" class="go-button">
            <span>GO</span>
          </button>
        </div>

        <div class="controls">
          <div class="control-group">
            <label class="control-label">Search Radius</label>
            <Slider
              :value="sliderRef"
              @update:modelValue="sliderRef = $event"
              :max="500"
              :step="5"
            />
            <div class="slider-value">
              <span v-if="sliderRef">{{ sliderRef[0] }} km</span>
            </div>
          </div>

          <button
            @click="getSkiAreas(posRef)"
            class="load-button"
            :disabled="waitingForApi"
          >
            <Spinner v-if="waitingForApi" />
            <span v-else>Load Ski Areas</span>
          </button>

          <div v-if="errorRef" class="error-message">
            {{ errorRef }}
          </div>
        </div>
      </div>

      <footer class="sidebar-footer">
        <p>© 2026 Ski Explorer</p>
      </footer>
    </aside>

    <!-- Main content -->
    <main class="main-content">
      <MapView ref="mapRef" class="map" :pos="posRef" />
    </main>
  </div>
</template>

<style scoped>
.app-root {
  position: relative;
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
  box-shadow: 2px 0 8px var(--shadow-color);
  border-right: 1px solid var(--border-color);
}

.sidebar-content {
  padding: 2rem 1.5rem;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.logo-section {
  text-align: center;
}

.logo {
  width: 100px;
  height: 100px;
  object-fit: contain;
  margin-bottom: 1rem;
  margin-left: auto;
  margin-right: auto;
}

.app-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  color: var(--title-color);
}

.controls {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.control-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--label-color);
}

.slider-value {
  text-align: center;
  font-size: 0.875rem;
  color: var(--label-color);
  font-weight: 500;
  margin-top: 0.5rem;
}

.go-button {
  padding: 0.875rem 1.25rem;
  border-radius: 8px;
  border: none;
  font-size: 1rem;
  font-weight: 500;
  background-color: var(--button-bg);
  color: var(--button-text);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  min-height: 44px;
}

.load-button {
  padding: 0.875rem 1.25rem;
  border-radius: 8px;
  border: none;
  font-size: 1rem;
  font-weight: 500;
  background-color: var(--button-bg);
  color: var(--button-text);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  min-height: 44px;
}

.load-button:hover:not(:disabled) {
  background-color: var(--button-hover);
}

.load-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  padding: 0.75rem;
  background-color: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 6px;
  color: var(--error-text);
  font-size: 0.875rem;
  line-height: 1.4;
}

.sidebar-footer {
  padding: 1.5rem;
  text-align: center;
  border-top: 1px solid var(--border-color);
}

.sidebar-footer p {
  margin: 0;
  font-size: 0.813rem;
  color: var(--footer-color);
}

/* Main content */
.main-content {
  flex: 1;
  height: 100%;
  padding: 1rem;
  background-color: var(--main-bg);
}

.map {
  width: 100%;
  height: 100%;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px var(--shadow-color);
}

.text-input {
  padding: 0.5rem;
  border-radius: 6px;
  border: 1px solid var(--border-color);
  font-size: 0.9rem;
  width: 100%;
  background-color: var(--input-bg);
  color: var(--text-color);
}

.data {
  position: absolute;
  right: 50px;
  left: auto;
  z-index: 500;
}
.leaflet-container {
  z-index: 0;
}

.sidebar {
  z-index: 10;
}

.sheet-content {
  z-index: 9999;
}
</style>
