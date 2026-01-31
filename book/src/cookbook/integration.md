# Integration Examples

Integrate spatial-narrative with other tools and workflows.

## Web Mapping

### Leaflet

Export to GeoJSON and display with Leaflet:

```rust
use spatial_narrative::io::{GeoJsonFormat, Format};

let geojson = GeoJsonFormat::new().export_str(&narrative)?;
std::fs::write("public/events.geojson", &geojson)?;
```

```html
<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
</head>
<body>
    <div id="map" style="height: 100vh;"></div>
    <script>
        const map = L.map('map').setView([40.7128, -74.0060], 10);
        
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '© OpenStreetMap'
        }).addTo(map);
        
        fetch('events.geojson')
            .then(res => res.json())
            .then(data => {
                L.geoJSON(data, {
                    pointToLayer: (feature, latlng) => {
                        return L.circleMarker(latlng, {
                            radius: 8,
                            fillColor: '#ff7800',
                            color: '#000',
                            weight: 1,
                            fillOpacity: 0.8
                        });
                    },
                    onEachFeature: (feature, layer) => {
                        layer.bindPopup(`
                            <strong>${feature.properties.text}</strong><br>
                            ${feature.properties.timestamp}
                        `);
                    }
                }).addTo(map);
            });
    </script>
</body>
</html>
```

### Mapbox GL JS

```javascript
mapboxgl.accessToken = 'your-token';
const map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/streets-v11',
    center: [-74.0060, 40.7128],
    zoom: 10
});

map.on('load', () => {
    map.addSource('events', {
        type: 'geojson',
        data: 'events.geojson'
    });
    
    map.addLayer({
        id: 'events-layer',
        type: 'circle',
        source: 'events',
        paint: {
            'circle-radius': 8,
            'circle-color': '#ff7800'
        }
    });
});
```

## Data Science

### Python/Pandas

Export to CSV for pandas:

```rust
use spatial_narrative::io::{CsvFormat, Format};

let csv = CsvFormat::new().export_str(&narrative)?;
std::fs::write("events.csv", &csv)?;
```

```python
import pandas as pd
import geopandas as gpd

# Read CSV
df = pd.read_csv('events.csv', parse_dates=['timestamp'])

# Basic analysis
print(f"Events: {len(df)}")
print(f"Date range: {df['timestamp'].min()} to {df['timestamp'].max()}")

# Convert to GeoDataFrame
gdf = gpd.GeoDataFrame(
    df, 
    geometry=gpd.points_from_xy(df.longitude, df.latitude),
    crs="EPSG:4326"
)

# Spatial operations
gdf.plot()
```

### R

```r
library(tidyverse)
library(sf)

# Read CSV
events <- read_csv("events.csv")

# Convert to sf object
events_sf <- st_as_sf(events, 
    coords = c("longitude", "latitude"), 
    crs = 4326)

# Plot
ggplot(events_sf) +
    geom_sf() +
    theme_minimal()
```

## GIS Software

### QGIS

1. Export to GeoJSON:
```rust
let geojson = GeoJsonFormat::new().export_str(&narrative)?;
std::fs::write("events.geojson", &geojson)?;
```

2. In QGIS: Layer → Add Layer → Add Vector Layer
3. Select the GeoJSON file

### ArcGIS

Export to GeoJSON, then import as feature class.

## Graph Visualization

### Graphviz

```rust
let dot = graph.to_dot();
std::fs::write("graph.dot", &dot)?;
```

```bash
# Render to PNG
dot -Tpng graph.dot -o graph.png

# Render to SVG
dot -Tsvg graph.dot -o graph.svg

# Different layouts
neato -Tpng graph.dot -o graph-neato.png
circo -Tpng graph.dot -o graph-circo.png
```

### D3.js Force Graph

```rust
let json = graph.to_json();
std::fs::write("public/graph.json", &json)?;
```

```html
<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
    fetch('graph.json')
        .then(res => res.json())
        .then(data => {
            const svg = d3.select("svg");
            const width = +svg.attr("width");
            const height = +svg.attr("height");
            
            const simulation = d3.forceSimulation(data.nodes)
                .force("link", d3.forceLink(data.edges).id(d => d.id))
                .force("charge", d3.forceManyBody().strength(-100))
                .force("center", d3.forceCenter(width / 2, height / 2));
            
            // Draw edges
            const link = svg.selectAll(".link")
                .data(data.edges)
                .join("line")
                .attr("class", "link");
            
            // Draw nodes
            const node = svg.selectAll(".node")
                .data(data.nodes)
                .join("circle")
                .attr("class", "node")
                .attr("r", 8);
            
            simulation.on("tick", () => {
                link.attr("x1", d => d.source.x)
                    .attr("y1", d => d.source.y)
                    .attr("x2", d => d.target.x)
                    .attr("y2", d => d.target.y);
                
                node.attr("cx", d => d.x)
                    .attr("cy", d => d.y);
            });
        });
</script>
```

## Database Integration

### SQLite/SQLx

```rust
use sqlx::sqlite::SqlitePool;

async fn save_events(pool: &SqlitePool, events: &[Event]) -> Result<()> {
    for event in events {
        sqlx::query(
            "INSERT INTO events (id, lat, lon, timestamp, text) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(event.id.to_string())
        .bind(event.location.lat)
        .bind(event.location.lon)
        .bind(event.timestamp.to_rfc3339())
        .bind(&event.text)
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn load_events(pool: &SqlitePool) -> Result<Vec<Event>> {
    let rows = sqlx::query("SELECT * FROM events")
        .fetch_all(pool)
        .await?;
    
    let events = rows.iter()
        .map(|row| Event::new(
            Location::new(row.get("lat"), row.get("lon")),
            Timestamp::parse(row.get::<String, _>("timestamp")).unwrap(),
            row.get("text")
        ))
        .collect();
    
    Ok(events)
}
```

### PostgreSQL with PostGIS

```sql
CREATE TABLE events (
    id UUID PRIMARY KEY,
    location GEOGRAPHY(POINT, 4326),
    timestamp TIMESTAMPTZ,
    text TEXT,
    tags TEXT[]
);

-- Spatial index
CREATE INDEX events_location_idx ON events USING GIST (location);
```

## REST API

### Actix-web

```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use spatial_narrative::io::{GeoJsonFormat, Format};

async fn get_events(data: web::Data<AppState>) -> HttpResponse {
    let geojson = GeoJsonFormat::new()
        .export_str(&data.narrative)
        .unwrap();
    
    HttpResponse::Ok()
        .content_type("application/geo+json")
        .body(geojson)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/events", web::get().to(get_events))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## CLI Tools

### Process Pipeline

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
    
    #[arg(short, long)]
    output: PathBuf,
    
    #[arg(long)]
    format: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load
    let input = std::fs::read_to_string(&args.input)?;
    let narrative = JsonFormat::new().import_str(&input)?;
    
    // Export
    let output = match args.format.as_str() {
        "geojson" => GeoJsonFormat::new().export_str(&narrative)?,
        "csv" => CsvFormat::new().export_str(&narrative)?,
        _ => JsonFormat::new().export_str(&narrative)?,
    };
    
    std::fs::write(&args.output, &output)?;
    Ok(())
}
```
