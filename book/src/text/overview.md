# Text Processing Overview

The `spatial_narrative` library provides comprehensive text processing capabilities for extracting geographic information from unstructured text.

## Modules

Text processing is split across two modules:

- **`text`** - Named Entity Recognition (NER), ML-NER, and keyword extraction
- **`parser`** - Geoparsing and coordinate detection

## Key Features

### Geoparsing (`parser` module)

Extract locations from text using multiple strategies:

- **Coordinate Detection**: Decimal degrees, degrees with symbols, DMS format
- **Place Name Resolution**: Built-in gazetteer with 2500+ world cities from GeoNames
- **Custom Gazetteers**: Plug in your own place name databases or external APIs

```rust
use spatial_narrative::parser::{GeoParser, BuiltinGazetteer};

let gazetteer = BuiltinGazetteer::new();
let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

let text = "The conference in Paris started at 48.8566°N, 2.3522°E.";
let mentions = parser.extract(text);

for mention in mentions {
    println!("Found: {} ({:?})", mention.text, mention.mention_type);
    if let Some(loc) = mention.location {
        println!("  -> {}, {}", loc.lat, loc.lon);
    }
}
```

### Named Entity Recognition (`text` module)

Extract entities from narrative text using rule-based patterns:

```rust
use spatial_narrative::text::TextAnalyzer;

let analyzer = TextAnalyzer::new();
let text = "Dr. Smith visited Google headquarters in Mountain View on January 15, 2024.";
let entities = analyzer.entities(text);

for entity in entities {
    println!("{}: {} (confidence: {:.2})", entity.entity_type, entity.text, entity.confidence);
}
```

### Keyword Extraction

Identify key terms and phrases:

```rust
use spatial_narrative::text::KeywordExtractor;

let extractor = KeywordExtractor::new();
let text = "Climate change affects coastal cities. Rising sea levels threaten coastal communities.";
let keywords = extractor.extract(text, 5);

for kw in keywords {
    println!("{}: {:.3}", kw.word, kw.score);
}
```

### ML-NER (Advanced, requires `ml-ner` feature)

Use transformer-based models for high-accuracy entity extraction:

```rust
use spatial_narrative::text::{MlNerModel, NerModel};

// Auto-download and cache model (~65MB)
let model = MlNerModel::download_blocking(NerModel::DistilBertQuantized)?;

let text = "Dr. Chen presented her findings in Paris on March 15, 2024.";
let entities = model.extract(text)?;

for entity in entities {
    println!("{}: \"{}\" (confidence: {:.2})", entity.label, entity.text, entity.score);
}
// Output:
// PER: "Dr. Chen" (confidence: 0.99)
// LOC: "Paris" (confidence: 0.98)
// MISC: "March 15, 2024" (confidence: 0.95)
```

## When to Use Each Module

| Task | Module | Key Type |
|------|--------|----------|
| Extract coordinates from text | `parser` | `GeoParser` |
| Resolve place names to coordinates | `parser` | `BuiltinGazetteer` |
| Extract entities (rule-based) | `text` | `TextAnalyzer` |
| Extract entities (ML, high accuracy) | `text` | `MlNerModel` |
| Find important keywords | `text` | `KeywordExtractor` |

## Next Steps

- [Geoparsing](./geoparser.md) - Coordinate detection and place name resolution
- [Named Entity Recognition](./analyzer.md) - Rule-based entity extraction
- [ML-NER (Advanced)](./ml-ner.md) - Machine learning-powered NER with auto-download
- [Keyword Extraction](./keywords.md) - Identify key terms
