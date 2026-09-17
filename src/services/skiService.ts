import { invoke } from "@tauri-apps/api/core";
import type { Position } from "../types/position";

export interface SkiArea {
  id: number;
  name: string;
  lat: number;
  lon: number;
  difficulty: string;
  operator?: string;
  is_bike: boolean;
  geometry: number[][];
}

export async function loadSkiAreas(
  position: Position,
  radius = 50000,
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

    throw new Error("OSM API überlastet");
  }
}

// skiService.ts
export async function logPisteCoordinates(
  lat: number,
  lon: number,
  name: string,
): Promise<void> {
  try {
    await invoke("log_piste_coordinates", { lat, lon, name });
  } catch (error) {
    console.error("Failed to log piste coordinates:", error);
    throw error;
  }
}

export async function loadBikeRoutes(
  position: Position,
  radius = 50000,
): Promise<SkiArea[]> {
  try {
    return await invoke<SkiArea[]>("fetch_bike_routes", {
      position,
      radius,
    });
  } catch (error) {
    console.error("Failed to load bike routes:", error);
    if (error instanceof Error) throw error;
    throw new Error("OSM API überlastet");
  }
}
