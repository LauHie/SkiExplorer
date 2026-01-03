import { invoke } from "@tauri-apps/api/core";

export interface SkiArea {
  id: number;
  name: string;
  lat: number;
  lon: number;
  operator?: string;
}

export async function loadSkiAreas(
  lat: number,
  lon: number,
  radius = 50000
): Promise<SkiArea[]> {
  try {
    const skiAreas = await invoke<SkiArea[]>("fetch_ski_areas", {
      lat,
      lon,
      radius,
    });
    return skiAreas;
  } catch (error) {
    console.error("Failed to load ski areas:", error);
    // Optionally, you can return an empty array instead of throwing
    return [];
    // Or rethrow if you want the caller to handle it
    // throw error
  }
}
