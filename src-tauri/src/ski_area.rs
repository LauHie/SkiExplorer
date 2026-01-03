use serde::Serialize;
use tauri::command;
use reqwest::Client;

#[derive(Serialize, Debug)]
pub struct SkiArea {
    id: i64,
    name: String,
    lat: f64,
    lon: f64,
    operator: Option<String>,
}

#[tauri::command]
pub async fn fetch_ski_areas(lat: f64, lon: f64, radius: u32) -> Result<Vec<SkiArea>, String> {
    let query = format!(
        r#"
        [out:json][timeout:25];
        (
          way(around:{radius},{lat},{lon})["piste:type"="downhill"];
        );
        out center tags;
        "#,
        lat = lat,
        lon = lon,
        radius = radius
    );

    let client = Client::new();
    let res = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    let mut ski_areas = Vec::new();

    if let Some(elements) = json["elements"].as_array() {
        for el in elements {
            let tags = &el["tags"];

            let name = tags["name"]
                .as_str()
                .unwrap_or("Unbenanntes Skigebiet")
                .to_string();

            let lat = el["lat"]
                .as_f64()
                .or_else(|| el["center"]["lat"].as_f64());

            let lon = el["lon"]
                .as_f64()
                .or_else(|| el["center"]["lon"].as_f64());

            if let (Some(lat), Some(lon)) = (lat, lon) {
                ski_areas.push(SkiArea {
                    id: el["id"].as_i64().unwrap_or(0),
                    name,
                    lat,
                    lon,
                    operator: tags["operator"].as_str().map(|s: &str | s.to_string()),
                });
            }
        }
    }

    // println!("{:?}", &ski_areas);

    Ok(ski_areas)
}

