# Keyword Extraction

The `KeywordExtractor` identifies important terms and phrases from text using term frequency analysis.

## Basic Usage

```rust
use spatial_narrative::text::KeywordExtractor;

let extractor = KeywordExtractor::new();
let text = "Climate change affects coastal cities. Rising sea levels threaten coastal communities worldwide.";
let keywords = extractor.extract(text, 5);

for kw in keywords {
    println!("{}: {:.3}", kw.text, kw.score);
}
```

## How It Works

The extractor uses a TF (Term Frequency) based approach:

1. **Tokenization**: Text is split into words
2. **Normalization**: Words are lowercased
3. **Filtering**: Stop words and short words are removed
4. **Scoring**: Words are scored by frequency
5. **Ranking**: Top N keywords are returned

## Keyword Struct

Each extracted keyword contains:

```rust
use spatial_narrative::text::{KeywordExtractor, Keyword};

let extractor = KeywordExtractor::new();
let keywords = extractor.extract("Machine learning is transforming technology.", 3);

for kw in keywords {
    println!("Word: {}", kw.text);
    println!("Score: {:.3}", kw.score);
    println!("Frequency: {}", kw.frequency);
}
```

| Field | Type | Description |
|-------|------|-------------|
| `text` | `String` | The keyword text |
| `score` | `f64` | Normalized score (0.0 to 1.0) |
| `frequency` | `usize` | Raw occurrence count |

## Configuration

### Minimum Word Length

Skip short words:

```rust
use spatial_narrative::text::KeywordExtractor;

let extractor = KeywordExtractor::new().min_length(4); // Skip words shorter than 4 chars

let text = "The big cat sat on the mat.";
let keywords = extractor.extract(text, 10);
// Only extracts words with 4+ characters
```

### Custom Stop Words

Add domain-specific stop words:

```rust
use spatial_narrative::text::KeywordExtractor;

let extractor = KeywordExtractor::new();

let text = "Data analysis reveals important data patterns in analysis.";
let keywords = extractor.extract_with_stopwords(text, 5, &["data", "analysis"]);
// "data" and "analysis" are now filtered out
```

### Phrase Extraction

Extract multi-word phrases (n-grams):

```rust
use spatial_narrative::text::KeywordExtractor;

// Configure max phrase length (includes bigrams and trigrams)
let extractor = KeywordExtractor::new().max_phrase_length(2);
let text = "Machine learning and deep learning are subfields of artificial intelligence.";
let phrases = extractor.extract(text, 3); // Top 3 terms (including bigrams)

for phrase in phrases {
    println!("{}: {:.3}", phrase.text, phrase.score);
}
// Example: "machine learning", "deep learning", "artificial intelligence"
```

The `max_phrase_length` parameter controls n-gram extraction (2 for bigrams, 3 for trigrams).

## Built-in Stop Words

The default stop word list includes common English words:

```text
the, a, an, is, are, was, were, be, been, being,
have, has, had, do, does, did, will, would, could,
should, may, might, must, shall, can, need, dare,
and, or, but, if, then, else, when, where, why,
how, all, each, every, both, few, more, most, other,
some, such, no, not, only, same, so, than, too, very,
just, also, now, here, there, this, that, these, those,
i, you, he, she, it, we, they, me, him, her, us, them,
my, your, his, its, our, their, mine, yours, hers, ours,
what, which, who, whom, whose, of, in, to, for, with,
on, at, by, from, as, into, through, during, before,
after, above, below, between, under, again, further,
once, about, up, down, out, off, over, own, because
```

## Use Cases

### Document Summarization

Extract key themes from a document:

```rust
use spatial_narrative::text::KeywordExtractor;

let extractor = KeywordExtractor::new();
let document = "Long document text...";
let themes = extractor.extract(document, 10);

println!("Key themes:");
for theme in themes {
    println!("  - {}", theme.text);
}
```

### Narrative Tagging

Auto-generate tags for narratives:

```rust
use spatial_narrative::text::KeywordExtractor;
use spatial_narrative::core::Narrative;

let extractor = KeywordExtractor::new();

fn auto_tag(narrative: &Narrative, extractor: &KeywordExtractor) -> Vec<String> {
    // Combine all event descriptions
    let text: String = narrative.events()
        .filter_map(|e| e.description.clone())
        .collect::<Vec<_>>()
        .join(" ");
    
    // Extract top keywords as tags
    extractor.extract(&text, 5)
        .into_iter()
        .map(|kw| kw.text)
        .collect()
}
```

### Topic Comparison

Compare topics between documents:

```rust
use spatial_narrative::text::KeywordExtractor;
use std::collections::HashSet;

let extractor = KeywordExtractor::new();

let doc1_keywords: HashSet<_> = extractor.extract(doc1, 20)
    .into_iter()
    .map(|kw| kw.word)
    .collect();

let doc2_keywords: HashSet<_> = extractor.extract(doc2, 20)
    .into_iter()
    .map(|kw| kw.word)
    .collect();

let common: HashSet<_> = doc1_keywords.intersection(&doc2_keywords).collect();
println!("Common topics: {:?}", common);
```

## Performance

The extractor is optimized for speed:

- O(n) tokenization
- O(n log n) sorting for top-k selection
- Minimal memory allocation

Typical performance: ~10,000 words/ms on modern hardware.

## Limitations

- English-focused (stop word list is English)
- No stemming or lemmatization
- TF-only (no IDF weighting)
- Case-insensitive matching only

For advanced keyword extraction with TF-IDF or semantic analysis, consider using specialized NLP libraries.
