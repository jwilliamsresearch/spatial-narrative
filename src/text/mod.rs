//! Natural language processing utilities.
//!
//! This module provides text analysis tools for working with
//! narrative text, including entity extraction and keyword analysis.
//!
//! # Overview
//!
//! The text module includes:
//!
//! - [`TextAnalyzer`] - Pattern-based named entity recognition for locations, organizations, etc.
//! - [`KeywordExtractor`] - Extract keywords and key phrases from text
//! - [`Entity`] - A detected named entity with type and span info
//! - [`Keyword`] - An extracted keyword with relevance score
//!
//! ## ML-based NER (Optional)
//!
//! With the `ml-ner` feature enabled, you get access to:
//!
//! - `MlNerModel` - Transformer-based NER using ONNX models
//! - `MlEntity` - Entity with confidence scores from ML inference
//!
//! Enable with: `spatial-narrative = { version = "0.1", features = ["ml-ner"] }`
//!
//! # Examples
//!
//! ## Extracting Named Entities
//!
//! ```rust
//! use spatial_narrative::text::{TextAnalyzer, EntityType};
//!
//! let mut analyzer = TextAnalyzer::new();
//! analyzer.add_location("Berlin");
//! let text = "The meeting was held in Berlin.";
//!
//! let entities = analyzer.entities(text);
//! assert!(entities.iter().any(|e| e.text == "Berlin"));
//! ```
//!
//! ## Extracting Keywords
//!
//! ```rust
//! use spatial_narrative::text::KeywordExtractor;
//!
//! let extractor = KeywordExtractor::new();
//! let text = "The earthquake caused significant damage to infrastructure. \
//!             Emergency responders worked through the night.";
//!
//! let keywords = extractor.extract(text, 5);
//! assert!(!keywords.is_empty());
//! ```
//!
//! ## Tokenization
//!
//! ```rust
//! use spatial_narrative::text::TextAnalyzer;
//!
//! let analyzer = TextAnalyzer::new();
//! let tokens = analyzer.tokenize("Hello, world! This is a test.");
//!
//! assert!(tokens.contains(&"Hello".to_string()));
//! assert!(tokens.contains(&"world".to_string()));
//! ```

mod analyzer;
mod entity;
mod keywords;

#[cfg(feature = "ml-ner")]
mod ml_ner;

pub use analyzer::TextAnalyzer;
pub use entity::{Entity, EntityType};
pub use keywords::{Keyword, KeywordExtractor};

#[cfg(feature = "ml-ner")]
pub use ml_ner::{init_ort, MlEntity, MlNerModel, MlNerResult};
