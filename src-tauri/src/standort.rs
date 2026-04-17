use std::ptr::null;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::types::Position;

#[derive(Serialize, Debug)]
pub struct Standort {
    pub id: i64,
    pub name: String,
    pub pos: Position,
    pub class: String,
}

#[tauri::command]
pub async fn get_standort(query_string: String) -> Result<Vec<Standort>, String> {
    let client = Client::new();

    // ✅ Nominatim statt Overpass
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=10",
        query_string
    );

    let res = client
        .get(&url)
        .header("User-Agent", "ski-explorer-app") // ⚠️ Pflicht!
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let text = res.text().await.map_err(|e| format!("Body error: {}", e))?;

    println!(
        "RAW RESPONSE:\n{}",
        serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&text).unwrap_or_default()
        )
        .unwrap_or(text.clone())
    );

    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("JSON parse error: {}", e))?;

    let mut results = Vec::new();

    if let Some(arr) = json.as_array() {
        for el in arr {
            let name = el
                .get("display_name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unbekannt")
                .to_string();

            let id = el
                .get("osm_id")
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                .unwrap_or(0);

            let lat = el
                .get("lat")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<f64>().ok());

            let lon = el
                .get("lon")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<f64>().ok());

            let class = el
                .get("class")
                .and_then(|v| v.as_str())
                .unwrap_or("Unbekannt")
                .to_string();

            if let (Some(lat), Some(lon)) = (lat, lon) {
                results.push(Standort {
                    id, // Nominatim hat keine ID → optional ersetzen
                    name,
                    pos: Position { lat, lon },
                    class,
                });
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&results).unwrap());
    Ok(results)
}
