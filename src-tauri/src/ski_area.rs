use crate::types::Position;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)] //Serialize um an Front-End zu senden
pub struct SkiArea {
    pub id: i64,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub difficulty: String,
    pub operator: Option<String>,
    pub is_bike: bool,
    pub geometry: Vec<Vec<f64>>,
}

// Helper enthält die komplette Abruf- und Parse-Logik
async fn fetch_overpass(query: String) -> Result<Vec<SkiArea>, String> {
    let url = format!("https://overpass-api.de/api/interpreter?{}", query);

    let client = Client::new();
    let mut last_err = None;

    for attempt in 1..=3 {
        match client
            .get(&url)
            .header("User-Agent", "ski-explorer-app (your@email.com)")
            .header("Accept", "application/json")
            .send()
            .await
        {
            Ok(res) => {
                let text = match res.text().await {
                    Ok(t) => t,
                    Err(e) => {
                        last_err = Some(format!("Failed to read response body: {}", e));
                        continue;
                    }
                };

                if !text.trim().starts_with('{') {
                    last_err = Some(format!("Overpass returned non-JSON: {}", text));
                    continue;
                }

                let json: Value = match serde_json::from_str(&text) {
                    Ok(j) => j,
                    Err(e) => {
                        last_err = Some(format!("Failed to parse JSON: {}", e));
                        continue;
                    }
                };

                let mut ski_areas = Vec::new();
                if let Some(elements) = json.get("elements").and_then(|e| e.as_array()) {
                    for el in elements {
                        let tags = el.get("tags").unwrap_or(&Value::Null);

                        let is_bike = tags.get("mtb:scale").is_some()
                            || tags
                                .get("bicycle")
                                .and_then(|v| v.as_str())
                                .map(|v| v == "designated")
                                .unwrap_or(false);

                        let default_name = if is_bike {
                            "Unbenannte Bike Strecke"
                        } else {
                            "Unbekannte Piste"
                        };
                        let name = tags
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or(default_name)
                            .to_string();

                        let mut lat = el
                            .get("center")
                            .and_then(|c| c.get("lat"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let mut lon = el
                            .get("center")
                            .and_then(|c| c.get("lon"))
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);

                        let difficulty = tags
                            .get("piste:difficulty")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unbekannt")
                            .to_string();

                        let geometry: Vec<Vec<f64>> = el
                            .get("geometry")
                            .and_then(|g| g.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|p| {
                                        let plat = p.get("lat")?.as_f64()?;
                                        let plon = p.get("lon")?.as_f64()?;
                                        Some(vec![plat, plon])
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        // Kein "center" vorhanden (bei "out geom") → Mittelpunkt der Linie
                        if lat == 0.0 && lon == 0.0 && !geometry.is_empty() {
                            let mid = &geometry[geometry.len() / 2];
                            lat = mid[0];
                            lon = mid[1];
                        }

                        // ❌ HIER WAR FRÜHER die is_bike-Berechnung – jetzt gelöscht

                        if lat != 0.0 && lon != 0.0 {
                            ski_areas.push(SkiArea {
                                id: el.get("id").and_then(|v| v.as_i64()).unwrap_or(0),
                                name,
                                lat,
                                lon,
                                difficulty,
                                operator: tags
                                    .get("operator")
                                    .and_then(|v| v.as_str())
                                    .map(String::from),
                                is_bike,
                                geometry,
                            });
                        }
                    }
                }

                println!("✅ Overpass: {} Ergebnisse geladen", ski_areas.len());

                return Ok(ski_areas);
            }
            Err(e) => last_err = Some(format!("HTTP request failed (attempt {}): {}", attempt, e)),
        }
    }
    Err(last_err.unwrap_or_else(|| "Unknown Overpass error".to_string()))
}

#[tauri::command]
pub async fn fetch_ski_areas(position: Position, radius: u32) -> Result<Vec<SkiArea>, String> {
    let query = format!(
        "data=[out:json][timeout:25];(way(around:{},{},{})[%22piste:type%22=%22downhill%22];);out center tags;",
        radius, position.lat, position.lon
    );
    fetch_overpass(query).await
}

#[tauri::command]
pub async fn fetch_bike_routes(position: Position, radius: u32) -> Result<Vec<SkiArea>, String> {
    let query = format!(
        "data=[out:json][timeout:25];(way(around:{},{},{})[%22mtb:scale%22];);out geom tags;",
        radius, position.lat, position.lon
    );
    fetch_overpass(query).await
}

#[tauri::command]
pub fn log_piste_coordinates(lat: f64, lon: f64, name: String) {
    println!(
        "Piste '{}' angeklickt: Latitude = {}, Longitude = {}",
        name, lat, lon
    );
    // Hier kannst du später weitere Logik hinzufügen (z. B. Datenbankabfrage)
}

#[tauri::command]
pub async fn fetch_route(
    start_lat: f64,
    start_lon: f64,
    end_lat: f64,
    end_lon: f64,
) -> Result<Vec<Vec<f64>>, String> {
    println!(
        "fetch_route aufgerufen: start=({}, {}), end=({}, {})",
        start_lat, start_lon, end_lat, end_lon
    );

    let url = format!(
        "https://router.project-osrm.org/route/v1/driving/{},{};{},{}?overview=full&geometries=geojson",
        start_lon, start_lat, end_lon, end_lat
    );

    println!("🌐 Routing-URL: {}", url);

    let client = Client::new();
    let res = match client
        .get(&url)
        .header("User-Agent", "ski-explorer-app (your@email.com)")
        .send()
        .await
    {
        Ok(r) => {
            println!("📡 HTTP Status: {}", r.status());
            r
        }
        Err(e) => {
            println!("❌ HTTP request failed: {}", e);
            return Err(format!("HTTP request failed: {}", e));
        }
    };

    let text = match res.text().await {
        Ok(t) => {
            println!("📄 Raw response: {}", t);
            t
        }
        Err(e) => {
            println!("❌ Failed to read body: {}", e);
            return Err(format!("Failed to read response body: {}", e));
        }
    };

    let json: Value = match serde_json::from_str(&text) {
        Ok(j) => j,
        Err(e) => {
            println!("❌ JSON parse error: {}", e);
            return Err(format!("Failed to parse JSON: {}", e));
        }
    };

    let code = json.get("code").and_then(|c| c.as_str()).unwrap_or("?");
    println!("🔍 OSRM code: {}", code);

    if code != "Ok" {
        return Err(format!("OSRM-Fehler: {}", text));
    }

    let coords = match json
        .get("routes")
        .and_then(|r| r.as_array())
        .and_then(|r| r.first())
        .and_then(|r| r.get("geometry"))
        .and_then(|g| g.get("coordinates"))
        .and_then(|c| c.as_array())
    {
        Some(c) => {
            println!("✅ {} Koordinatenpunkte erhalten", c.len());
            c
        }
        None => {
            println!("❌ Keine Koordinaten in der Antwort gefunden");
            return Err("Keine Route in der Antwort gefunden".to_string());
        }
    };

    let route: Vec<Vec<f64>> = coords
        .iter()
        .filter_map(|c| {
            let arr = c.as_array()?;
            let lon = arr.first()?.as_f64()?;
            let lat = arr.get(1)?.as_f64()?;
            Some(vec![lat, lon])
        })
        .collect();

    println!(
        "Route fertig: {} Punkte | erster: {:?} | letzter: {:?}",
        route.len(),
        route.first(),
        route.last()
    );

    Ok(route)
}
