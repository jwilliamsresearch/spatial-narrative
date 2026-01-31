# CSV Format

Export narratives as CSV for spreadsheet analysis and data science workflows.

## Basic Usage

```rust
use spatial_narrative::io::{CsvFormat, Format};

// Export to CSV
let csv = CsvFormat::new().export_str(&narrative)?;

// Import from CSV
let narrative = CsvFormat::new().import_str(&csv)?;
```

## Output Structure

Default CSV output:

```csv
latitude,longitude,timestamp,text,tags
40.7128,-74.006,2024-01-15T10:00:00Z,"Conference begins","conference,technology"
40.758,-73.9855,2024-01-15T14:00:00Z,"Press conference","press,media"
```

## Configuration Options

Customize column names and delimiter:

```rust
use spatial_narrative::io::{CsvFormat, CsvOptions};

let options = CsvOptions {
    lat_column: "lat".to_string(),
    lon_column: "lng".to_string(),
    timestamp_column: "datetime".to_string(),
    text_column: Some("description".to_string()),
    tags_column: Some("categories".to_string()),
    elevation_column: Some("altitude".to_string()),
    source_title_column: None,
    source_url_column: None,
    delimiter: b',',
};

let format = CsvFormat::with_options(options);
let csv = format.export_str(&narrative)?;
```

### Column Options

| Option | Default | Description |
|--------|---------|-------------|
| `lat_column` | `"latitude"` | Latitude column name |
| `lon_column` | `"longitude"` | Longitude column name |
| `timestamp_column` | `"timestamp"` | Timestamp column name |
| `text_column` | `Some("text")` | Event text column |
| `tags_column` | `Some("tags")` | Tags column (comma-separated) |
| `elevation_column` | `None` | Elevation column |
| `source_title_column` | `None` | Source title column |
| `source_url_column` | `None` | Source URL column |
| `delimiter` | `b','` | Field delimiter |

## TSV (Tab-Separated)

```rust
let options = CsvOptions {
    delimiter: b'\t',
    ..Default::default()
};

let tsv = CsvFormat::with_options(options).export_str(&narrative)?;
```

## Importing CSV

Import from existing CSV data:

```rust
let csv = r#"lat,lon,time,description
40.7128,-74.006,2024-01-15T10:00:00Z,Conference begins
40.758,-73.9855,2024-01-15T14:00:00Z,Press conference"#;

let options = CsvOptions {
    lat_column: "lat".to_string(),
    lon_column: "lon".to_string(),
    timestamp_column: "time".to_string(),
    text_column: Some("description".to_string()),
    ..Default::default()
};

let narrative = CsvFormat::with_options(options).import_str(csv)?;
```

## Handling Tags

Tags are stored as comma-separated values:

```rust
// Export: tags become "tag1,tag2,tag3"
// Import: "tag1,tag2,tag3" becomes HashSet{"tag1", "tag2", "tag3"}

let csv = "latitude,longitude,timestamp,text,tags
40.7128,-74.006,2024-01-15T10:00:00Z,Event,\"conference,technology,important\"";

let narrative = CsvFormat::new().import_str(csv)?;
assert!(narrative.events[0].has_tag("conference"));
```

## File Operations

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

// Export to file
let file = File::create("events.csv")?;
CsvFormat::new().export(&narrative, &mut BufWriter::new(file))?;

// Import from file
let file = File::open("events.csv")?;
let narrative = CsvFormat::new().import(&mut BufReader::new(file))?;
```

## Integration with Data Tools

### Python/Pandas

```python
import pandas as pd

# Read exported CSV
df = pd.read_csv('events.csv', parse_dates=['timestamp'])

# Analyze
print(f"Events: {len(df)}")
print(f"Date range: {df['timestamp'].min()} to {df['timestamp'].max()}")
print(f"Locations: {df[['latitude', 'longitude']].nunique()}")
```

### Excel

CSV files open directly in Excel. For best results:
- Use UTF-8 encoding
- Quote text fields containing commas
- Use ISO 8601 timestamps

### R

```r
library(tidyverse)

events <- read_csv("events.csv")

# Plot on map
library(sf)
events_sf <- st_as_sf(events, coords = c("longitude", "latitude"), crs = 4326)
plot(events_sf)
```

## When to Use CSV

**Use CSV when:**
- Analyzing data in spreadsheets (Excel, Google Sheets)
- Using with data science tools (pandas, R)
- Simple data exchange with minimal overhead
- Data pipelines that expect tabular formats

**Consider other formats when:**
- You need complex metadata → use [JSON](./json.md)
- Mapping in web applications → use [GeoJSON](./geojson.md)
- Source attribution is important → use [JSON](./json.md)

## Limitations

CSV has some inherent limitations:

| Feature | Support |
|---------|---------|
| Location coordinates | ✅ Full |
| Timestamps | ✅ Full |
| Event text | ✅ Full |
| Tags | ✅ As comma-separated string |
| Basic source info | ⚠️ Optional columns |
| Custom metadata | ❌ Not supported |
| Nested data | ❌ Not supported |
