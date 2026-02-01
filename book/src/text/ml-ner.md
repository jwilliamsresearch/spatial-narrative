# Machine Learning Named Entity Recognition

The `ml-ner` feature provides state-of-the-art Named Entity Recognition using transformer models (BERT, RoBERTa, DistilBERT) via ONNX Runtime.

## Features

- **High Accuracy**: Transformer-based models trained on CoNLL-2003
- **Auto-Download**: Automatically fetch models from HuggingFace Hub
- **Caching**: Models are cached locally after first download
- **Multiple Models**: Choose from 5 pre-trained models or bring your own
- **Multi-language**: Support for 40+ languages with the multilingual model

## Entity Types

All pre-trained models recognize four entity types:

- **LOC** - Locations (cities, countries, regions)
- **PER** - Persons (names of people)
- **ORG** - Organizations (companies, institutions)
- **MISC** - Miscellaneous (dates, events, products)

## Installation

Add the `ml-ner-download` feature to enable both ML-NER and auto-download:

```toml
[dependencies]
spatial-narrative = { version = "0.1", features = ["ml-ner-download"] }
```

Or use just `ml-ner` if you want to provide your own models:

```toml
[dependencies]
spatial-narrative = { version = "0.1", features = ["ml-ner"] }
```

## ONNX Runtime Setup

ML-NER requires ONNX Runtime to be installed. You have several options:

### Option 1: Environment Variable

Set `ORT_DYLIB_PATH` to point to your ONNX Runtime library:

```bash
# macOS
export ORT_DYLIB_PATH=/path/to/libonnxruntime.dylib

# Linux
export ORT_DYLIB_PATH=/path/to/libonnxruntime.so

# Windows
set ORT_DYLIB_PATH=C:\path\to\onnxruntime.dll
```

### Option 2: Install via Package Manager

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

### Option 3: Manual Download

Download from [ONNX Runtime releases](https://github.com/microsoft/onnxruntime/releases):

1. Download the appropriate package for your platform
2. Extract the archive
3. Set `ORT_DYLIB_PATH` to the library file

## Available Models

| Model | Size | F1 Score | Speed | Languages |
|-------|------|----------|-------|-----------|
| **DistilBertQuantized** | ~65MB | ~90% | Fast | English |
| **DistilBert** | ~250MB | ~90% | Fast | English |
| **BertBase** | ~400MB | ~91% | Medium | English |
| **BertLarge** | ~1.2GB | ~93% | Slow | English |
| **Multilingual** | ~700MB | ~90% | Medium | 40+ languages |

The **DistilBertQuantized** model is recommended for most use cases, offering the best balance of size, speed, and accuracy.

## Basic Usage

### Auto-Download (Recommended)

The simplest way to get started:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

// First run downloads ~65MB, subsequent runs load from cache
let model = MlNerModel::download_blocking(NerModel::DistilBertQuantized)?;

let text = "Dr. Sarah Chen presented her research in Paris on March 15, 2024.";
let entities = model.extract(text)?;

for entity in entities {
    println!("{}: \"{}\" (confidence: {:.2})", 
        entity.label, entity.text, entity.score);
}
// Output:
// PER: "Dr. Sarah Chen" (confidence: 0.99)
// LOC: "Paris" (confidence: 0.98)
// MISC: "March 15, 2024" (confidence: 0.95)
```

### With Progress Reporting

Show download progress for large models:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

let model = MlNerModel::download_blocking_with_progress(
    NerModel::DistilBertQuantized,
    |downloaded, total| {
        if total > 0 {
            let pct = (downloaded as f64 / total as f64) * 100.0;
            println!("Downloading: {:.1}%", pct);
        }
    }
)?;
```

### Using Different Models

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

// For best accuracy (larger download)
let model = MlNerModel::download_blocking(NerModel::BertLarge)?;

// For multilingual text
let model = MlNerModel::download_blocking(NerModel::Multilingual)?;

// For custom HuggingFace models
let model = MlNerModel::download_blocking(
    NerModel::Custom("my-org/my-ner-model".into())
)?;
```

## Advanced Usage

### Manual Model Loading

If you have pre-downloaded ONNX models:

```rust
use spatial_narrative::text::MlNerModel;

let model = MlNerModel::from_directory("./my-ner-model/")?;
// Directory should contain: model.onnx, tokenizer.json, config.json
```

### Cache Management

```rust
use spatial_narrative::text::{
    model_cache_dir,
    model_cache_path,
    is_model_cached,
    cache_size_bytes,
    clear_model_cache,
    NerModel,
};

// Check cache location
println!("Cache dir: {:?}", model_cache_dir());

// Check if a model is cached
let model = NerModel::DistilBertQuantized;
if is_model_cached(&model) {
    println!("Model already cached at: {:?}", model_cache_path(&model));
}

// Get total cache size
if let Ok(size) = cache_size_bytes() {
    println!("Cache size: {:.2} MB", size as f64 / 1024.0 / 1024.0);
}

// Clear cache for a specific model
clear_model_cache(Some(&model))?;

// Clear all cached models
clear_model_cache(None)?;
```

### Async API

For async applications, use the async API:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

let model = MlNerModel::download(NerModel::DistilBertQuantized).await?;
let entities = model.extract("Text to analyze")?;
```

## Integration with Geoparsing

Combine ML-NER with gazetteer lookup for comprehensive geoparsing:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};
use spatial_narrative::parser::{BuiltinGazetteer, Gazetteer};

// Extract entities with ML
let ml_model = MlNerModel::download_blocking(NerModel::DistilBertQuantized)?;
let text = "The summit was held in Geneva, Switzerland.";
let ml_entities = ml_model.extract(text)?;

// Resolve locations with gazetteer
let gazetteer = BuiltinGazetteer::new();

for ml_entity in ml_entities {
    if ml_entity.label == "LOC" {
        // Convert to standard Entity and lookup coordinates
        let entity = ml_entity.to_entity();
        
        if let Some(location) = gazetteer.lookup(&entity.text) {
            println!("{} is at {}, {}", 
                entity.text, location.lat, location.lon);
        }
    }
}
```

## Entity Structure

The `MlEntity` struct provides detailed extraction results:

```rust
pub struct MlEntity {
    /// Entity type: "LOC", "PER", "ORG", or "MISC"
    pub label: String,
    
    /// The extracted text
    pub text: String,
    
    /// Confidence score (0.0 to 1.0)
    pub score: f64,
    
    /// Character position in original text
    pub start: usize,
    
    /// End position in original text
    pub end: usize,
}
```

Convert to standard `Entity` for use with other components:

```rust
let entity = ml_entity.to_entity();
// Returns Entity with appropriate EntityType enum
```

## Example Application

See the complete example:

```bash
cargo run --example ml_ner_download --features ml-ner-download
```

This example demonstrates:
- Checking cache status
- Auto-downloading models
- Extracting entities from various texts
- Integration with geoparsing workflow

## Exporting Custom Models

To use your own fine-tuned models:

1. **Train or fine-tune** a token classification model on HuggingFace
2. **Export to ONNX** using Optimum:

```bash
pip install optimum[exporters]
optimum-cli export onnx --model your-model-name ./output-dir/
```

3. **Load in spatial-narrative**:

```rust
let model = MlNerModel::from_directory("./output-dir/")?;
```

Or host on HuggingFace Hub and use:

```rust
let model = MlNerModel::download_blocking(
    NerModel::Custom("your-org/your-model".into())
)?;
```

## Performance Tips

1. **Choose the right model**: Use DistilBertQuantized for most applications
2. **Cache models**: First download takes time, but subsequent loads are fast
3. **Batch processing**: Process multiple texts in sequence after loading once
4. **Model lifecycle**: Keep the model in memory for repeated extractions
5. **Async for I/O**: Use async API when downloading in web servers

## Troubleshooting

### ONNX Runtime Not Found

If you see errors about ONNX Runtime:

1. Install ONNX Runtime (see setup section above)
2. Set `ORT_DYLIB_PATH` environment variable
3. Verify the path points to the correct library file

### Model Download Fails

- Check internet connection
- Verify HuggingFace Hub is accessible
- Try clearing cache: `clear_model_cache(None)?`
- Check cache directory permissions

### Low Accuracy

- Try a larger model (BertBase or BertLarge)
- For non-English text, use the Multilingual model
- Consider fine-tuning a custom model on your domain

## Cache Locations

Models are cached in platform-specific directories:

- **Linux**: `~/.cache/spatial-narrative/models/`
- **macOS**: `~/Library/Caches/spatial-narrative/models/`
- **Windows**: `%LOCALAPPDATA%\spatial-narrative\models\`

Each model has its own subdirectory containing:
- `model.onnx` - The neural network model
- `tokenizer.json` - Text tokenization configuration
- `config.json` - Label mappings and metadata

## License Notes

- **DistilBERT models**: Apache 2.0 License
- **BERT models**: Apache 2.0 License
- **Multilingual model**: CC BY-NC-SA 4.0 License (non-commercial)
- **ONNX Runtime**: MIT License

Check individual model licenses on HuggingFace Hub before commercial use.
