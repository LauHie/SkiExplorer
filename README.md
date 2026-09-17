# Ski Explorer

Plattformübergreifende App zur Visualisierung von Ski-Pisten und Mountainbike-Routen
(Desktop: Tauri + Rust | Mobile: Tauri Mobile + Rust | Kartendaten: OpenStreetMap-API)

---

## Features
- Ski-Pisten anzeigen (Farbskala: Blau = leicht, Gelb = mittel, Rot = schwer)
- Mountainbike-Routen als gelb-schwarze Linien darstellen
- Standortsuche per Stadtname oder Drag & Drop auf der Karte
- Suchradius anpassbar (2–100 km)
- Routing von Pisten/Bike-Markern zum aktuellen Standort
- Plattformübergreifend: Windows (Desktop) + Android (Mobile)

---

## Technologien

| Bereich       | Technologie          | Version       | Zweck                          |
|---------------|----------------------|---------------|--------------------------------|
| Frontend      | Tauri (Desktop)      | 2.x           | Cross-Plattform UI Framework   |
|               | Tauri Mobile         | 2.x           | Android-Unterstützung          |
|               | Vue.js               | 3.x           | UI-Komponenten                 |
|               | TypeScript           | 5.x           | Typisierung für Vue            |
| Backend       | Rust                 | 1.7x          | Logik, API-Anbindung           |
| Karten        | OpenStreetMap (OSM)  | –             | Kartendaten & Routen           |
| APIs          | OSM Overpass API     | –             | Pisten/Bike-Routen abfragen    |
|               | OSRM API             | –             | Routing-Berechnungen           |

---

## Recommended IDE Setup
- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

## Installation und Ausführung

### Desktop (Windows)

**Voraussetzungen:**
- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- Node.js (optional, falls Vue.js direkt genutzt wird)

**Projekt klonen und starten:**
```bash
git clone https://github.com/LauHie/SkiExplorer.git
cd SkiExplorer
bun install
bun tauri dev  # Startet die App im Entwicklungsmodus
