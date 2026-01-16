use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct Position{
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize)]
pub struct SkiArea {
    pub id: i64,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub operator: Option<String>,
}

#[tauri::command]
pub async fn fetch_ski_areas(
    position: Position,
    radius: u32,
) -> Result<Vec<SkiArea>, String> {
    let query = format!(
        r#"
        [out:json][timeout:25];
        (
          way(around:{radius},{lat},{lon})["piste:type"="downhill"];
        );
        out center tags;
        "#,
        lat = position.lat,
        lon = position.lon,
        radius = radius
    );

    let client = Client::new();
    let mut last_err = None;

    for attempt in 1..=3 {
        match client
            .post("https://overpass-api.de/api/interpreter")
            .body(query.clone())
            .send()
            .await
        {
            Ok(res) => {
                // Clone headers info first to avoid borrow issues
                let content_type = res
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string(); // <- clone the string

                // Now safely move res to read the body
                let text = match res.text().await {
                    Ok(t) => t,
                    Err(e) => {
                        last_err = Some(format!("Failed to read response body: {}", e));
                        continue;
                    }
                };

                // Check if it's JSON
                if !content_type.contains("application/json") {
                    last_err = Some(format!(
                        "Overpass returned non-JSON (attempt {}): {}",
                        attempt, text
                    ));
                    continue; // retry
                }

                // Parse JSON
                let json: Value = match serde_json::from_str(&text) {
                    Ok(j) => j,
                    Err(e) => {
                        last_err = Some(format!("Failed to parse JSON: {}", e));
                        continue;
                    }
                };

                // Build SkiArea list
                let mut ski_areas = Vec::new();
                if let Some(elements) = json.get("elements").and_then(|e| e.as_array()) {
                    for el in elements {
                        let tags = el.get("tags").unwrap_or(&Value::Null);

                        let name = tags
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unbenanntes Skigebiet")
                            .to_string();

                        let lat = el
                            .get("lat")
                            .and_then(|v| v.as_f64())
                            .or_else(|| el.get("center").and_then(|c| c.get("lat")).and_then(|v| v.as_f64()));

                        let lon = el
                            .get("lon")
                            .and_then(|v| v.as_f64())
                            .or_else(|| el.get("center").and_then(|c| c.get("lon")).and_then(|v| v.as_f64()));

                        if let (Some(lat), Some(lon)) = (lat, lon) {
                            ski_areas.push(SkiArea {
                                id: el.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
                                name,
                                lat,
                                lon,
                                operator: tags.get("operator").and_then(|v| v.as_str()).map(String::from),
                            });
                        }
                    }
                }

                return Ok(ski_areas);
            }
            Err(e) => last_err = Some(format!("HTTP request failed (attempt {}): {}", attempt, e)),
        }
    }

    // All retries failed
    Err(last_err.unwrap_or_else(|| "Unknown Overpass error".to_string()))
}
