import { invoke } from "@tauri-apps/api/core";
import type { Position } from "../types/position";

export interface SkiArea {
  id: number;
  name: string;
  lat: number;
  lon: number;
  operator?: string;
}

export async function loadSkiAreas(
  position: Position,
  radius = 50000
): Promise<SkiArea[]> {
  try {
    return await invoke<SkiArea[]>("fetch_ski_areas", {
      position,
      radius,
    });
  } catch (error) {
    console.error("Failed to load ski areas:", error);

    // Normalize error to Error instance
    if (error instanceof Error) {
      throw error;
    }

    throw new Error("OSM API überlastet weil kacke");
  }
}
