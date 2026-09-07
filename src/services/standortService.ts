import { invoke } from "@tauri-apps/api/core";
import type { Position } from "../types/position";

export interface Standort {
  id: number;
  name: string;
  pos: Position;
  class: String;
  boundary?: [number, number, number, number];
}

export async function loadStandort(queryString: String): Promise<Standort[]> {
  try {
    return await invoke<Standort[]>("get_standort", {
      queryString,
    });
  } catch (error) {
    console.error("Failed to load standort:", error);

    // Normalize error to Error instance
    if (error instanceof Error) {
      throw error;
    }

    throw new Error("Standort nicht abgerufen");
  }
}
