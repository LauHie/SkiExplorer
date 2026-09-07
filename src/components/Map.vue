<template>
  <div class="relative z-[1] h-full w-full">
    <div id="map" class="h-full w-full"></div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount } from "vue";
import L from "leaflet";
import type { SkiArea } from "../services/skiService";
import type { Standort } from "../services/standortService";
import { Position } from "../types/position";

let map: L.Map;
let pisteLayer: L.LayerGroup;
let userLayer: L.LayerGroup;
let standorteLayer: L.LayerGroup;
let boundaryLayer: L.LayerGroup;

function initMap(pos: Position) {
  map = L.map("map").setView([pos.lat, pos.lon], 9);

  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "© OpenStreetMap contributors",
  }).addTo(map);

  boundaryLayer = L.layerGroup().addTo(map);
  pisteLayer = L.layerGroup().addTo(map);
  standorteLayer = L.layerGroup().addTo(map);
  userLayer = L.layerGroup().addTo(map);
}

function setView(pos: Position, zoom: number = 11) {
  if (!Number.isFinite(pos.lat) || !Number.isFinite(pos.lon)) return;
  if (Math.abs(pos.lat) > 90 || Math.abs(pos.lon) > 180) return;
  map.setView([pos.lat, pos.lon], zoom);
  addUserMarker(pos);
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

function showStandorte(standorte: Standort[], selectedId?: number) {
  standorteLayer.clearLayers();
  standorte.forEach((s, i) => {
    if (
      !Number.isFinite(s.pos.lat) ||
      !Number.isFinite(s.pos.lon) ||
      Math.abs(s.pos.lat) > 90 ||
      Math.abs(s.pos.lon) > 180
    )
      return;

    const isSelected = s.id === selectedId;
    const color = isSelected ? "#ef4444" : "#2563eb";
    const icon = L.divIcon({
      className: "standort-pin",
      html: `<div style="display:flex;align-items:center;justify-content:center;width:28px;height:28px;border-radius:9999px;border:2px solid #fff;background:${color};color:#fff;font-size:12px;font-weight:600;box-shadow:0 2px 6px rgba(0,0,0,.35);">${i + 1}</div>`,
      iconSize: [28, 28],
      iconAnchor: [14, 14],
    });
    L.marker([s.pos.lat, s.pos.lon], { icon })
      .addTo(standorteLayer)
      .bindPopup(`<b>${s.name}</b>`);
  });
}

function clearStandorte() {
  standorteLayer.clearLayers();
}

function bboxToGeoJSON(boundary: [number, number, number, number]) {
  const [minLat, maxLat, minLon, maxLon] = boundary;

  return {
    type: "Feature",
    geometry: {
      type: "Polygon",
      coordinates: [
        [
          [minLon, minLat],
          [maxLon, minLat],
          [maxLon, maxLat],
          [minLon, maxLat],
          [minLon, minLat], // schließen
        ],
      ],
    },
    properties: {},
  };
}

function setBoundary(boundary?: [number, number, number, number]) {
  boundaryLayer.clearLayers();
  if (!boundary) return;
  const bound = bboxToGeoJSON(boundary);
  const layer = L.geoJSON(bound as any, {
    style: {
      color: "#2563eb",
      weight: 4, // vorher 2 → dicker
      opacity: 1, // vorher 0.9
      fillColor: "#3b82f6",
      fillOpacity: 0.25, // vorher 0.15 → stärker sichtbar
    },
  }).addTo(boundaryLayer);

  try {
    map.fitBounds(layer.getBounds(), { padding: [40, 40] });
    map.addLayer(boundaryLayer);

    map = L.map("map", {
      preferCanvas: false,
    });

    map.createPane("boundaryPane");
    map.getPane("boundaryPane")!.style.zIndex = "650";
  } catch {
    // ignore invalid geometry
  }
  L.geoJSON(bound as any, {
    pane: "boundaryPane",
    style: {
      color: "#2563eb",
      weight: 4,
    },
  }).addTo(boundaryLayer);
}

function clearBoundary() {
  boundaryLayer.clearLayers();
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
  showStandorte,
  clearStandorte,
  setBoundary,
  clearBoundary,
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
