# Named Entity Recognition

The `TextAnalyzer` provides basic Named Entity Recognition (NER) for extracting structured information from narrative text.

## Basic Usage

```rust
use spatial_narrative::text::TextAnalyzer;

let analyzer = TextAnalyzer::new();
let text = "Dr. Smith met with CEO Johnson at Google headquarters.";
let entities = analyzer.entities(text);

for entity in entities {
    println!("{:?}: {}", entity.entity_type, entity.text);
}
```

## Entity Types

The analyzer detects six entity types:

| Type | Description | Examples |
|------|-------------|----------|
| `Person` | People's names | "Dr. Smith", "Mr. Johnson" |
| `Organization` | Companies, institutions | "Google Inc.", "MIT", "NASA" |
| `Location` | Place names | "New York", "Mount Everest" |
| `DateTime` | Dates and times | "January 15, 2024", "March 2023" |
| `Numeric` | Numbers with units | "$5.5 million", "100 km" |
| `Event` | Named events | (custom additions) |
| `Other` | Unclassified entities | - |

## Person Detection

Detects names with common titles:

```rust
use spatial_narrative::text::{TextAnalyzer, EntityType};

let analyzer = TextAnalyzer::new();
let text = "Dr. Jane Smith and Prof. Bob Johnson attended the meeting.";
let entities = analyzer.entities(text);

let people: Vec<_> = entities.iter()
    .filter(|e| matches!(e.entity_type, EntityType::Person))
    .collect();

for person in people {
    println!("Person: {}", person.text);
}
```

Recognized titles: Dr., Mr., Mrs., Ms., Miss, Prof., Professor, President, Senator, Governor, Mayor, Chief, Captain, General, Admiral, etc.

## Organization Detection

Detects organizations by suffix patterns:

```rust
use spatial_narrative::text::{TextAnalyzer, EntityType};

let analyzer = TextAnalyzer::new();
let text = "Apple Inc. partnered with MIT and the World Health Organization.";
let entities = analyzer.entities(text);

let orgs: Vec<_> = entities.iter()
    .filter(|e| matches!(e.entity_type, EntityType::Organization))
    .collect();
// Found: "Apple Inc.", "MIT", "World Health Organization"
```

Recognized patterns:
- Suffixes: Inc., Corp., LLC, Ltd., Co., Foundation, Institute, University, Organization
- Acronyms: NASA, FBI, CIA, NATO, WHO, UN, EU, etc.

## Date Detection

Extracts various date formats:

```rust
use spatial_narrative::text::{TextAnalyzer, EntityType};

let analyzer = TextAnalyzer::new();
let text = "The event on January 15, 2024 was rescheduled to March 2024.";
let entities = analyzer.entities(text);

let dates: Vec<_> = entities.iter()
    .filter(|e| matches!(e.entity_type, EntityType::DateTime))
    .collect();
```

Recognized formats:
- Full dates: "January 15, 2024", "15 January 2024"
- Month-year: "March 2024", "Jan 2024"
- Abbreviated months: "Jan", "Feb", "Mar", etc.

## Location Detection

Detects common location patterns and major places:

```rust
use spatial_narrative::text::{TextAnalyzer, EntityType};

let analyzer = TextAnalyzer::new();
let text = "The company has offices in New York, London, and Tokyo.";
let entities = analyzer.entities(text);

let locations: Vec<_> = entities.iter()
    .filter(|e| matches!(e.entity_type, EntityType::Location))
    .collect();
```

Built-in location database includes major world cities and countries.

## Numeric Detection

Extracts numbers with units:

```rust
use spatial_narrative::text::{TextAnalyzer, EntityType};

let analyzer = TextAnalyzer::new();
let text = "The project cost $5.5 million and covered 100 kilometers.";
let entities = analyzer.entities(text);

let numerics: Vec<_> = entities.iter()
    .filter(|e| matches!(e.entity_type, EntityType::Numeric))
    .collect();
// Found: "$5.5 million", "100 kilometers"
```

Recognized patterns:
- Currency: "$5.5 million", "€100"
- Distance: "100 km", "50 miles"
- Percentages: "25%", "75 percent"
- General: "1,000", "3.14"

## Tokenization

The analyzer also provides text tokenization:

```rust
use spatial_narrative::text::TextAnalyzer;

let analyzer = TextAnalyzer::new();
let text = "Hello, world! This is a test.";

// All tokens
let tokens = analyzer.tokenize(text);
// ["Hello", ",", "world", "!", "This", "is", "a", "test", "."]

// Words only (no punctuation)
let words = analyzer.tokenize_words(text);
// ["Hello", "world", "This", "is", "a", "test"]
```

## Sentence Splitting

Split text into sentences:

```rust
use spatial_narrative::text::TextAnalyzer;

let analyzer = TextAnalyzer::new();
let text = "First sentence. Second sentence! Third sentence?";
let sentences = analyzer.sentences(text);

assert_eq!(sentences.len(), 3);
```

## Custom Locations

Add custom location names:

```rust
use spatial_narrative::text::TextAnalyzer;

let mut analyzer = TextAnalyzer::new();
analyzer.add_location("Springfield");
analyzer.add_location("Gotham City");

let text = "The hero saved Gotham City.";
let entities = analyzer.entities(text);
// Now detects "Gotham City" as a location
```

## Confidence Scores

Each entity has a confidence score:

```rust
use spatial_narrative::text::TextAnalyzer;

let analyzer = TextAnalyzer::new();
let entities = analyzer.entities("Dr. Smith works at NASA.");

for entity in entities {
    println!("{}: {} (confidence: {:.2})", 
        entity.entity_type, 
        entity.text, 
        entity.confidence
    );
}
```

Confidence levels:
- **0.9+**: High confidence (clear patterns like "Dr. Smith")
- **0.7-0.9**: Medium confidence
- **0.5-0.7**: Lower confidence (may need verification)

## Limitations

This is a rule-based NER system, not a machine learning model:

- ✅ Fast and deterministic
- ✅ No external dependencies
- ✅ Works offline
- ❌ May miss unconventional patterns
- ❌ Limited to English
- ❌ No context-aware disambiguation

For production NLP tasks requiring high accuracy, consider integrating with external NLP services or ML models.
