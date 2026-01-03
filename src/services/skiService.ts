import { invoke } from "@tauri-apps/api/core"

export interface SkiArea {
  id: number
  name: string
  lat: number
  lon: number
  operator?: string
}

export async function loadSkiAreas(
  lat: number,
  lon: number,
  radius = 50000
): Promise<SkiArea[]> {
  return await invoke<SkiArea[]>("fetch_ski_areas", {
    lat,
    lon,
    radius,
  })
}
