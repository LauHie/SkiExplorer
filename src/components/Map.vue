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
import { invoke } from "@tauri-apps/api/core";

let map: L.Map;
let pisteLayer: L.LayerGroup;
let userLayer: L.LayerGroup;
let standorteLayer: L.LayerGroup;
let routeLayer: L.LayerGroup;
let currentPos: Position | null = null; // merkt sich den gewählten Standort

const emit = defineEmits<{
  (e: "positionChanged", pos: Position): void;
}>();

function initMap(pos: Position) {
  map = L.map("map").setView([pos.lat, pos.lon], 9);

  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "© OpenStreetMap contributors",
  }).addTo(map);

  pisteLayer = L.layerGroup().addTo(map);
  standorteLayer = L.layerGroup().addTo(map);
  userLayer = L.layerGroup().addTo(map);
  routeLayer = L.layerGroup().addTo(map);
  currentPos = pos;
}

function setView(pos: Position, zoom: number = 11) {
  if (!Number.isFinite(pos.lat) || !Number.isFinite(pos.lon)) return;
  if (Math.abs(pos.lat) > 90 || Math.abs(pos.lon) > 180) return;
  currentPos = pos;
  map.setView([pos.lat, pos.lon], zoom);
  addUserMarker(pos);
}

function createStripedMarker(lat: number, lon: number) {
  const svg = `
    <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
      <!-- Schwarzer Kreis mit weißem Rand -->
      <circle cx="10" cy="10" r="8" fill="#000000" stroke="#ffffff" stroke-width="1.5"/>
      <!-- Gelbe diagonale Streifen (direkt gezeichnet, ohne pattern) -->
      <g stroke="#facc15" stroke-width="1.8" stroke-linecap="round">
        <line x1="5.5" y1="9.5" x2="9.5" y2="5.5"/>
        <line x1="6.5" y1="13.5" x2="13.5" y2="6.5"/>
        <line x1="10.5" y1="14.5" x2="14.5" y2="10.5"/>
      </g>
    </svg>
  `;

  const icon = L.divIcon({
    html: svg,
    className: "striped-marker",
    iconSize: [20, 20],
    iconAnchor: [10, 10],
  });

  return L.marker([lat, lon], { icon }).addTo(pisteLayer);
}

function createFreerideMarker(lat: number, lon: number) {
  const svg = `
    <svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
      <!-- Orange Diamant mit weißem Rand -->
      <path d="M10 1 L19 10 L10 19 L1 10 Z" fill="#f97316" stroke="#ffffff" stroke-width="1.5"/>
      <!-- Kleines weißes Berg-Symbol -->
      <path d="M6.5 12.5 L9 8.5 L10.5 10.5 L12 8 L13.5 12.5 Z" fill="#ffffff"/>
    </svg>
  `;

  const icon = L.divIcon({
    html: svg,
    className: "freeride-marker",
    iconSize: [20, 20],
    iconAnchor: [10, 10],
  });

  return L.marker([lat, lon], { icon }).addTo(pisteLayer);
}

function buildPopupHtml(piste: SkiArea, color: string, difficultyText: string) {
  return `
    <div style="
      min-width: 220px;
      padding: 12px;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      background: white;
      border-radius: 8px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    ">
      <div style="margin-bottom: 8px;">
        <b style="font-size: 16px; color: #1f2937;">${piste.name}</b>
      </div>
      <div style="margin-bottom: 12px; color: #6b7280; font-size: 14px;">
        Schwierigkeit: <span style="color: ${color}; font-weight: 600;">${difficultyText}</span>
      </div>
      <button
        style="
          width: 100%;
          padding: 10px 16px;
          background: linear-gradient(135deg, #2563eb, #1d4ed8);
          color: white;
          border: none;
          border-radius: 6px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s ease;
          box-shadow: 0 2px 4px rgba(37, 99, 235, 0.3);
        "
        onmouseover="this.style.background='linear-gradient(135deg, #1d4ed8, #1e40af)'; this.style.transform='translateY(-1px)'; this.style.boxShadow='0 4px 8px rgba(37, 99, 235, 0.4)'"
        onmouseout="this.style.background='linear-gradient(135deg, #2563eb, #1d4ed8)'; this.style.transform='translateY(0)'; this.style.boxShadow='0 2px 4px rgba(37, 99, 235, 0.3)'"
        onclick="window.__requestRoute(${piste.lat}, ${piste.lon})"
      >
        Route anzeigen
      </button>
    </div>
  `;
}

function addUserMarker(pos: Position) {
  userLayer.clearLayers();

  const marker = L.marker([pos.lat, pos.lon], { draggable: true }) // ✅ ziehbar
    .addTo(userLayer)
    .bindPopup("Dein Standort");

  marker.on("dragend", () => {
    const newPos = marker.getLatLng();

    routeLayer.clearLayers();

    // ✅ currentPos aktualisieren (wichtig fürs Routing!)
    currentPos = { lat: newPos.lat, lon: newPos.lng };

    // Popup zeigt die neuen Koordinaten
    marker.setPopupContent(
      `Dein Standort<br><small>${newPos.lat.toFixed(5)}, ${newPos.lng.toFixed(5)}</small>`,
    );

    // ✅ App.vue benachrichtigen (damit "Pisten laden" die neue Position nutzt)
    emit("positionChanged", { lat: newPos.lat, lon: newPos.lng });
  });
}

function addPisteMarkers(pistes: SkiArea[]) {
  pisteLayer.clearLayers();

  pistes.forEach((piste) => {
    const difficulty = piste.difficulty?.toLowerCase();
    const color = getDifficultyColor(difficulty);

    if (!difficulty || difficulty === "unbekannt" || difficulty === "unknown") {
      // ✅ Unbekannt → gestreifter Marker
      const marker = createStripedMarker(piste.lat, piste.lon);
      marker.bindPopup(buildPopupHtml(piste, color, "unbekannt ❓"));
    } else if (difficulty === "freeride") {
      // ✅ Freeride → Diamant-Marker
      const marker = createFreerideMarker(piste.lat, piste.lon);
      marker.bindPopup(buildPopupHtml(piste, color, "Freeride 🏔️"));
    } else {
      // ✅ Bekannte Schwierigkeit → normaler Kreis-Marker
      L.circleMarker([piste.lat, piste.lon], {
        radius: 6,
        color: color,
        fillColor: color,
        fillOpacity: 0.8,
      })
        .addTo(pisteLayer)
        .bindPopup(buildPopupHtml(piste, color, piste.difficulty));
    }
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

    // Marker erstellen und explizit kein Popup binden
    const marker = L.marker([s.pos.lat, s.pos.lon], { icon }).addTo(
      standorteLayer,
    );
    marker.unbindPopup(); // Stelle sicher, dass kein Popup existiert
  });
}

function clearStandorte() {
  standorteLayer.clearLayers();
}

function drawRoute(coords: number[][]) {
  routeLayer.clearLayers();

  // 1. Weiße "Umrandung" (Casing) – hebt die Linie vom Kartenhintergrund ab
  L.polyline(coords as L.LatLngExpression[], {
    color: "#ffffff",
    weight: 8, // breiter als die Route
    opacity: 0.85,
    lineCap: "round", // runde Enden
    lineJoin: "round", // runde Kurven
  }).addTo(routeLayer);

  // 2. Pinke Route darüber
  const polyline = L.polyline(coords as L.LatLngExpression[], {
    color: "#ec4899",
    weight: 4,
    opacity: 0.95,
    lineCap: "round",
    lineJoin: "round",
    className: "route-line", // Animation
  }).addTo(routeLayer);

  const last = coords[coords.length - 1];
  if (last) {
    L.circleMarker(last as L.LatLngExpression, {
      radius: 6,
      color: "#ffffff",
      fillColor: "#ec4899",
      fillOpacity: 1,
      weight: 2,
    }).addTo(routeLayer);
  }

  try {
    map.fitBounds(polyline.getBounds(), { padding: [50, 50] });
  } catch {
    // ignore invalid geometry
  }
}

function clearRoute() {
  routeLayer.clearLayers();
}

function getDifficultyColor(diff?: string) {
  const difficulty = diff?.toLowerCase();
  switch (difficulty) {
    case "easy":
    case "novice":
      return "#3b82f6"; // Blau (Anfänger)
    case "intermediate":
      return "#f59e0b"; // Gelb (Mittel)
    case "advanced":
    case "hard":
    case "expert":
    case "extreme":
    case "black":
    case "double_black":
      return "#ef4444"; // Rot (Schwer/Experte)
    case "freeride":
      return "#f97316"; // Orange (für die Anzeige im Popup)
    default:
      return "#9ca3af"; // Grau (unbekannt)
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
  setView,
});

onMounted(() => {
  initMap(props.pos);
  addUserMarker(props.pos);
  // ✅ Tauri global verfügbar machen (für Leaflet-Popups)
  (window as any).__tauriInvoke = invoke;

  (window as any).__requestRoute = async (lat: number, lon: number) => {
    if (!currentPos) {
      console.error(
        "Kein Standort gesetzt – bitte zuerst einen Standort auswählen.",
      );
      return;
    }
    try {
      const route = await invoke<number[][]>("fetch_route", {
        startLat: currentPos.lat, // JS camelCase → Rust snake_case (start_lat)
        startLon: currentPos.lon,
        endLat: lat,
        endLon: lon,
      });
      drawRoute(route);
    } catch (e) {
      console.error("Routing-Fehler:", e);
    }
  };
});

onBeforeUnmount(() => {
  map.remove();
  // ✅ Optional: Tauri wieder entfernen
  delete (window as any).__tauriInvoke;
  delete (window as any).__requestRoute;
});
</script>

<style>
.route-line {
  stroke-dasharray: 12 12;
  animation: route-dash 1.2s linear infinite;
}
@keyframes route-dash {
  from {
    stroke-dashoffset: 0;
  }
  to {
    stroke-dashoffset: -24;
  }
}
</style>
