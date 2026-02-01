# Installation

## Requirements

- **Rust**: 1.70 or later
- **Cargo**: Included with Rust

## Adding to Your Project

Add `spatial-narrative` to your `Cargo.toml`:

```toml
[dependencies]
spatial-narrative = "0.1"
```

Or use cargo add:

```bash
cargo add spatial-narrative
```

## Features

The library comes with sensible defaults. All core features are included by default.

### Default Features

```toml
[dependencies]
spatial-narrative = "0.1"  # Includes all standard features
```

### Optional Features

Enable additional functionality by specifying features:

```toml
[dependencies]
spatial-narrative = { version = "0.1", features = ["geocoding", "ml-ner-download"] }
```

| Feature | Description | Default |
|---------|-------------|---------|
| `serde` | Serialization/deserialization support | ✅ |
| `parallel` | Parallel processing with rayon | ❌ |
| `geocoding` | External geocoding APIs (Nominatim, GeoNames, Wikidata) | ❌ |
| `gpx-support` | GPX file format support | ❌ |
| `database` | Database persistence (PostgreSQL, SQLite) | ❌ |
| `projections` | Coordinate system transformations | ❌ |
| `nlp` | Enhanced text processing with NLP | ❌ |
| `ml-ner` | Machine learning NER with ONNX Runtime | ❌ |
| `ml-ner-download` | Auto-download ML models from HuggingFace Hub | ❌ |
| `cli` | Command-line interface tools | ❌ |
| `full` | All features enabled | ❌ |

#### ML-NER Requirements

The `ml-ner` and `ml-ner-download` features require ONNX Runtime:

**macOS (Homebrew):**
```bash
brew install onnxruntime
export ORT_DYLIB_PATH=$(brew --prefix onnxruntime)/lib/libonnxruntime.dylib
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt install libonnxruntime
export ORT_DYLIB_PATH=/usr/lib/libonnxruntime.so
```

**Manual Download:**
Download from [ONNX Runtime releases](https://github.com/microsoft/onnxruntime/releases) and set `ORT_DYLIB_PATH` to the library path.

See [ML-NER documentation](../text/ml-ner.md) for detailed setup instructions.

## Verifying Installation

Create a simple test file:

```rust
// src/main.rs
use spatial_narrative::core::{Location, Timestamp, Event};

fn main() {
    let location = Location::new(40.7128, -74.0060);
    let timestamp = Timestamp::now();
    let event = Event::new(location, timestamp, "Hello, spatial-narrative!");
    
    println!("Created event: {}", event.text);
    println!("Location: ({}, {})", event.location.lat, event.location.lon);
}
```

Run it:

```bash
cargo run
```

You should see:

```
Created event: Hello, spatial-narrative!
Location: (40.7128, -74.006)
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/spatial-narrative.git
cd spatial-narrative

# Build
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Minimum Supported Rust Version (MSRV)

The current MSRV is **Rust 1.70**.

This version is tested in CI and will be maintained according to our [compatibility policy](../reference/faq.md#compatibility).

## Next Steps

- [Quick Start](./quick-start.md) - Get up and running quickly
- [Concepts](./concepts.md) - Understand the core ideas
