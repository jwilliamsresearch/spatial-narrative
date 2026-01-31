# Text Processing Overview

The `spatial_narrative` library provides comprehensive text processing capabilities for extracting geographic information from unstructured text.

## Modules

Text processing is split across two modules:

- **`text`** - Named Entity Recognition (NER) and keyword extraction
- **`parser`** - Geoparsing and coordinate detection

## Key Features

### Geoparsing (`parser` module)

Extract locations from text using multiple strategies:

- **Coordinate Detection**: Decimal degrees, degrees with symbols, DMS format
- **Place Name Resolution**: Built-in gazetteer with 200+ major world locations
- **Custom Gazetteers**: Plug in your own place name databases

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

Extract entities from narrative text:

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

## When to Use Each Module

| Task | Module | Key Type |
|------|--------|----------|
| Extract coordinates from text | `parser` | `GeoParser` |
| Resolve place names to coordinates | `parser` | `BuiltinGazetteer` |
| Extract people, organizations, dates | `text` | `TextAnalyzer` |
| Find important keywords | `text` | `KeywordExtractor` |

## Next Steps

- [Geoparsing](./geoparser.md) - Coordinate detection and place name resolution
- [Named Entity Recognition](./analyzer.md) - Extract entities from text
- [Keyword Extraction](./keywords.md) - Identify key terms
