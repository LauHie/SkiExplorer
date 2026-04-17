<template>
  <div class="map-container">
    <div id="map"></div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import L from "leaflet";
import type { SkiArea } from "../services/skiService";
import { Position } from "../types/position";

let map: L.Map;
let pisteLayer: L.LayerGroup;
let userLayer: L.LayerGroup;

function initMap(pos: Position) {
  map = L.map("map").setView([pos.lat, pos.lon], 9);

  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "© OpenStreetMap contributors",
  }).addTo(map);

  pisteLayer = L.layerGroup().addTo(map);
  userLayer = L.layerGroup().addTo(map);
}

function setView(pos: Position) {
  if (pos.lat >= 0 && pos.lon >= 0) {
    map.setView([pos.lat, pos.lon], 9);
    addUserMarker(pos);
  }
}

function addUserMarker(pos: Position) {
  userLayer.clearLayers();
  L.marker([pos.lat, pos.lon]).addTo(userLayer).bindPopup("Dein Standort");
}

function addPisteMarkers(pistes: SkiArea[]) {
  pisteLayer.clearLayers();

  pistes.forEach((piste) => {
    L.circleMarker([piste.lat, piste.lon], {
      radius: 6,
      color: getDifficultyColor(piste.difficulty),
      fillOpacity: 0.8,
    }).addTo(pisteLayer).bindPopup(`
        <b>${piste.name}</b><br/>
        Schwierigkeit: ${piste.difficulty ?? "unbekannt"}
      `);
  });
}

function clearPisteMarkers() {
  pisteLayer.clearLayers();
}

function getDifficultyColor(diff?: string) {
  switch (diff) {
    case "blue":
      return "blue";
    case "red":
      return "red";
    case "black":
      return "black";
    default:
      return "gray";
  }
}
const props = defineProps<{
  pos: Position;
}>();

defineExpose({
  addUserMarker,
  addPisteMarkers,
  clearPisteMarkers,
  setView,
});

onMounted(() => {
  initMap(props.pos);
  addUserMarker(props.pos);
});

onBeforeUnmount(() => {
  map.remove();
});
</script>

<style scoped>
.map-container {
  width: 100%;
  height: 100%;
  z-index: 1 !important;
}

#map {
  width: 100%;
  height: 100%;
}
</style>
