<template>
  <div class="map-container">
    <div id="map"></div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue"
import L from "leaflet"
import type { SkiArea } from "../services/skiService"

let map: L.Map
let pisteLayer: L.LayerGroup
let userLayer: L.LayerGroup

function initMap() {
  map = L.map("map").setView([47.2682, 11.3923], 9)

  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "© OpenStreetMap contributors",
  }).addTo(map)

  pisteLayer = L.layerGroup().addTo(map)
  userLayer = L.layerGroup().addTo(map)
}

function addUserMarker(lat: number, lon: number) {
  userLayer.clearLayers()
  L.marker([lat, lon])
    .addTo(userLayer)
    .bindPopup("Dein Standort")
}

function addPisteMarkers(pistes: SkiArea[]) {
  pisteLayer.clearLayers()

  pistes.forEach((piste) => {
    L.circleMarker([piste.lat, piste.lon], {
      radius: 6,
      color: getDifficultyColor(piste.difficulty),
      fillOpacity: 0.8,
    })
      .addTo(pisteLayer)
      .bindPopup(`
        <b>${piste.name}</b><br/>
        Schwierigkeit: ${piste.difficulty ?? "unbekannt"}
      `)
  })
}

function clearPisteMarkers() {
  pisteLayer.clearLayers()
}

function getDifficultyColor(diff?: string) {
  switch (diff) {
    case "blue":
      return "blue"
    case "red":
      return "red"
    case "black":
      return "black"
    default:
      return "gray"
  }
}

defineExpose({
  addUserMarker,
  addPisteMarkers,
  clearPisteMarkers,
})

onMounted(() => {
  initMap()
  addUserMarker(47.2682, 11.3923)
})

onBeforeUnmount(() => {
  map.remove()
})
</script>

<style scoped>
.map-container {
  width: 100%;
  height: 100%;
}

#map {
  width: 100%;
  height: 100%;
}
</style>
