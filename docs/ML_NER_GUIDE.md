# ML-Based Named Entity Recognition

## Overview

The `ml-ner` feature provides state-of-the-art named entity recognition using transformer models (BERT, RoBERTa, DistilBERT, etc.) exported to ONNX format. This offers significantly better accuracy than pattern-based extraction, especially for:

- **Context-aware entity recognition** - Distinguishes "Apple" the company from "apple" the fruit
- **Complex entity mentions** - Handles multi-word entities and abbreviations
- **Ambiguous cases** - Uses surrounding context to determine entity types
- **Domain-specific entities** - Can be fine-tuned for specific domains (medical, legal, etc.)

## Quick Start

### 1. Download ONNX Runtime

Download the ONNX Runtime library for your platform:
- **Windows**: [onnxruntime-win-x64-*.zip](https://github.com/microsoft/onnxruntime/releases)
- **Linux**: [onnxruntime-linux-x64-*.tgz](https://github.com/microsoft/onnxruntime/releases)
- **macOS**: [onnxruntime-osx-arm64-*.tgz](https://github.com/microsoft/onnxruntime/releases) or x64

Extract and note the path to the library file:
- Windows: `onnxruntime.dll`
- Linux: `libonnxruntime.so`
- macOS: `libonnxruntime.dylib`

### 2. Export a BERT-NER Model

Install the required Python packages:
```bash
pip install optimum[exporters] torch transformers
```

Export a pre-trained BERT-NER model to ONNX:
```bash
# Popular choice: dslim/bert-base-NER (English, CoNLL-2003 labels)
optimum-cli export onnx --model dslim/bert-base-NER ./models/bert-ner-onnx/

# Other options:
# - dslim/bert-large-NER (more accurate, slower)
# - Jean-Baptiste/camembert-ner (French)
# - dbmdz/bert-large-cased-finetuned-conll03-english
```

This creates three files:
- `model.onnx` - The neural network
- `tokenizer.json` - Text preprocessing
- `config.json` - Label mappings

### 3. Set Environment Variable

**Windows PowerShell:**
```powershell
$env:ORT_DYLIB_PATH = "C:\path\to\onnxruntime\lib\onnxruntime.dll"
```

**Linux/macOS:**
```bash
export ORT_DYLIB_PATH=/path/to/libonnxruntime.so
```

### 4. Use in Your Code

```rust
use spatial_narrative::text::{init_ort, MlNerModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ONNX Runtime (or use ORT_DYLIB_PATH env var)
    init_ort("/path/to/onnxruntime")?;

    // Load the model
    let model = MlNerModel::from_directory("./models/bert-ner-onnx/")?;

    // Extract entities
    let text = "Dr. Jane Smith from Stanford University visited Paris last week.";
    let entities = model.extract(text)?;

    for entity in entities {
        println!(
            "{}: \"{}\" (confidence: {:.2})",
            entity.label,
            entity.text,
            entity.score
        );
    }

    Ok(())
}
```

## Expected Output

For the text: *"Dr. Jane Smith from Stanford University visited Paris last week."*

The model would extract:
```
👤 PER: "Jane Smith" (confidence: 0.98) [4:14]
🏢 ORG: "Stanford University" (confidence: 0.95) [20:39]
📍 LOC: "Paris" (confidence: 0.97) [48:53]
```

## Model Details

### Architecture: BERT (Bidirectional Encoder Representations from Transformers)

BERT is a transformer-based model that:
1. **Tokenizes** text into subword units using WordPiece tokenization
2. **Encodes** each token with bidirectional context (looks both left and right)
3. **Classifies** each token into entity labels using a classification head
4. **Aggregates** consecutive tokens with the same label into entities

### Label Schemes

**CoNLL-2003** (most common):
- `B-PER`, `I-PER` - Person names (Beginning/Inside)
- `B-ORG`, `I-ORG` - Organizations
- `B-LOC`, `I-LOC` - Locations
- `B-MISC`, `I-MISC` - Miscellaneous entities
- `O` - Outside any entity

**OntoNotes 5.0**:
- `PERSON`, `ORG`, `GPE` (Geo-Political Entity), `DATE`, `TIME`, `MONEY`, `PERCENT`, etc.

### Performance Characteristics

**Accuracy**:
- Pattern-based (TextAnalyzer): ~70-80% F1 score
- BERT-base NER: ~90-95% F1 score on CoNLL-2003
- BERT-large NER: ~92-96% F1 score

**Speed** (on modern CPU):
- ~50-100 tokens/second for BERT-base
- ~20-50 tokens/second for BERT-large

**Memory**:
- Model size: ~400MB (BERT-base), ~1.2GB (BERT-large)
- Runtime memory: ~500MB-1GB during inference

## Comparison: ML vs Pattern-Based

| Feature | ML-based (ml-ner) | Pattern-based (TextAnalyzer) |
|---------|-------------------|------------------------------|
| **Accuracy** | 90-95% F1 | 70-80% F1 |
| **Context awareness** | ✅ Yes | ❌ No |
| **Setup complexity** | High (model + runtime) | None |
| **Dependencies** | ONNX Runtime (~200MB) | None |
| **Speed** | Medium (50-100 tok/s) | Fast (10K+ tok/s) |
| **Model size** | 400MB-1.2GB | None |
| **Customizable** | ✅ Fine-tune on domain | Limited |
| **Languages** | Depends on model | English patterns only |

## Use Cases

**Use ML-based NER when:**
- Accuracy is critical (legal documents, medical records)
- You need context-aware extraction (distinguishing "Apple" company vs fruit)
- You have domain-specific requirements and can fine-tune models
- You're processing large volumes where quality matters more than speed

**Use pattern-based NER when:**
- You need fast, lightweight extraction
- Deployment environment has limited resources
- You only need basic entity detection
- You want zero external dependencies

## Integration with Spatial-Narrative

Both ML-based and pattern-based NER return `Entity` objects that integrate seamlessly with the rest of the spatial-narrative ecosystem:

```rust
use spatial_narrative::text::{MlNerModel, EntityType};
use spatial_narrative::parser::Gazetteer;
use spatial_narrative::core::{Event, Location};

// Extract entities with ML
let entities = model.extract(text)?;

// Filter for locations
let locations: Vec<_> = entities.iter()
    .filter(|e| e.to_entity().entity_type == EntityType::Location)
    .collect();

// Geocode with gazetteer
let gazetteer = Gazetteer::builtin();
for loc in locations {
    if let Some(coords) = gazetteer.geocode(&loc.text) {
        println!("{} -> {:?}", loc.text, coords);
    }
}
```

## Troubleshooting

### "Failed to initialize ONNX Runtime"
- Ensure `ORT_DYLIB_PATH` points to the correct library file
- Verify the library matches your system architecture (x64 vs ARM)
- On Windows, check that Visual C++ Redistributable is installed

### "Model file not found"
- Verify the directory contains `model.onnx`, `tokenizer.json`, `config.json`
- Check file permissions

### Slow inference
- Use BERT-base instead of BERT-large
- Enable GPU execution provider (requires CUDA/DirectML setup)
- Process texts in batches (future feature)

### Out of memory
- Use a smaller model (DistilBERT)
- Reduce batch size
- Limit input text length

## Advanced: Custom Models

You can use any HuggingFace token classification model:

```bash
# Medical NER
optimum-cli export onnx --model d4data/biomedical-ner-all ./models/bio-ner/

# Multilingual NER
optimum-cli export onnx --model xlm-roberta-large-finetuned-conll03-english ./models/xlm-ner/

# Your own fine-tuned model
optimum-cli export onnx --model ./my-custom-bert ./models/custom-ner/
```

Just ensure the model outputs logits with shape `[batch_size, sequence_length, num_labels]` and uses BIO/BILOU tagging scheme.
