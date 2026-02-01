//! ML-based Named Entity Recognition using ONNX Runtime.
//!
//! This module provides ML-powered NER using transformer models (BERT, RoBERTa, etc.)
//! exported to ONNX format. Requires the `ml-ner` feature.
//!
//! # Setup
//!
//! This module uses ONNX Runtime for inference. You have two options:
//!
//! 1. **Environment Variable**: Set `ORT_DYLIB_PATH` to the path of your ONNX Runtime library
//!    (e.g., `onnxruntime.dll` on Windows, `libonnxruntime.so` on Linux, `libonnxruntime.dylib` on macOS)
//!
//! 2. **Programmatic Initialization**: Call [`init_ort`] before creating any model:
//!    ```rust,ignore
//!    init_ort("/path/to/onnxruntime.dll")?;
//!    ```
//!
//! You can download ONNX Runtime binaries from:
//! <https://github.com/microsoft/onnxruntime/releases>
//!
//! # Model Requirements
//!
//! The module expects an ONNX model trained for token classification (NER).
//! Compatible models can be exported from HuggingFace using the Optimum library:
//!
//! ```bash
//! pip install optimum[exporters]
//! optimum-cli export onnx --model dslim/bert-base-NER ./bert-ner-onnx/
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use spatial_narrative::text::{init_ort, MlNerModel};
//!
//! // Initialize ONNX Runtime (or set ORT_DYLIB_PATH env var)
//! init_ort("path/to/onnxruntime")?;
//!
//! let model = MlNerModel::from_directory("./bert-ner-onnx/")?;
//! let entities = model.extract("Dr. Smith visited Paris on Monday.")?;
//!
//! for entity in entities {
//!     println!("{}: {} ({:.2})", entity.label, entity.text, entity.score);
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::Tensor;
use tokenizers::Tokenizer;

use super::entity::{Entity, EntityType};
use crate::error::Error;

/// Result type for ML NER operations.
pub type MlNerResult<T> = Result<T, Error>;

/// Initialize ONNX Runtime with a path to the library.
///
/// This function must be called before creating any [`MlNerModel`] instances,
/// unless the `ORT_DYLIB_PATH` environment variable is set.
///
/// # Arguments
///
/// * `dylib_path` - Path to the ONNX Runtime shared library:
///   - Windows: `onnxruntime.dll`
///   - Linux: `libonnxruntime.so`
///   - macOS: `libonnxruntime.dylib`
///
/// # Example
///
/// ```rust,ignore
/// use spatial_narrative::text::init_ort;
///
/// // On Windows
/// init_ort("C:/onnxruntime/lib/onnxruntime.dll")?;
///
/// // On Linux
/// init_ort("/usr/local/lib/libonnxruntime.so")?;
/// ```
pub fn init_ort<P: AsRef<Path>>(dylib_path: P) -> MlNerResult<()> {
    let env_builder = ort::init_from(dylib_path.as_ref())
        .map_err(|e| Error::ParseError(format!("Failed to initialize ONNX Runtime: {}", e)))?;
    
    let success = env_builder.commit();
    if !success {
        return Err(Error::ParseError(
            "Failed to commit ONNX Runtime environment".to_string(),
        ));
    }
    Ok(())
}

/// An entity detected by the ML model.
#[derive(Debug, Clone)]
pub struct MlEntity {
    /// The entity text
    pub text: String,
    /// The entity label (e.g., "PER", "LOC", "ORG")
    pub label: String,
    /// Confidence score (0.0 to 1.0)
    pub score: f32,
    /// Start character position
    pub start: usize,
    /// End character position
    pub end: usize,
}

impl MlEntity {
    /// Convert to the standard Entity type.
    pub fn to_entity(&self) -> Entity {
        let entity_type = match self.label.as_str() {
            "PER" | "B-PER" | "I-PER" | "PERSON" => EntityType::Person,
            "ORG" | "B-ORG" | "I-ORG" | "ORGANIZATION" => EntityType::Organization,
            "LOC" | "B-LOC" | "I-LOC" | "LOCATION" | "GPE" | "B-GPE" | "I-GPE" => {
                EntityType::Location
            }
            "DATE" | "B-DATE" | "I-DATE" | "TIME" | "B-TIME" | "I-TIME" => EntityType::DateTime,
            "MISC" | "B-MISC" | "I-MISC" => EntityType::Other,
            _ => EntityType::Other,
        };

        Entity::new(&self.text, entity_type, self.start, self.end)
            .with_confidence(self.score as f64)
    }
}

/// ML-based Named Entity Recognition model using ONNX Runtime.
///
/// Supports BERT-based NER models exported to ONNX format.
pub struct MlNerModel {
    session: Mutex<Session>,
    tokenizer: Tokenizer,
    id2label: HashMap<i64, String>,
}

impl MlNerModel {
    /// Load a model from a directory containing model.onnx and tokenizer files.
    ///
    /// The directory should contain:
    /// - `model.onnx` - The ONNX model file
    /// - `tokenizer.json` - The tokenizer configuration
    /// - `config.json` - Model configuration with id2label mapping
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let model = MlNerModel::from_directory("./bert-ner-onnx/")?;
    /// ```
    pub fn from_directory<P: AsRef<Path>>(dir: P) -> MlNerResult<Self> {
        let dir = dir.as_ref();

        // Load ONNX model
        let model_path = dir.join("model.onnx");
        if !model_path.exists() {
            return Err(Error::ParseError(format!(
                "Model file not found: {}",
                model_path.display()
            )));
        }

        let session = Session::builder()
            .map_err(|e| Error::ParseError(format!("Failed to create session: {}", e)))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| Error::ParseError(format!("Failed to set optimization level: {}", e)))?
            .commit_from_file(&model_path)
            .map_err(|e| Error::ParseError(format!("Failed to load model: {}", e)))?;

        // Load tokenizer
        let tokenizer_path = dir.join("tokenizer.json");
        if !tokenizer_path.exists() {
            return Err(Error::ParseError(format!(
                "Tokenizer file not found: {}",
                tokenizer_path.display()
            )));
        }

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| Error::ParseError(format!("Failed to load tokenizer: {}", e)))?;

        // Load config for id2label mapping
        let config_path = dir.join("config.json");
        let id2label = if config_path.exists() {
            Self::load_id2label(&config_path)?
        } else {
            // Default CoNLL-2003 labels
            Self::default_id2label()
        };

        Ok(Self {
            session: Mutex::new(session),
            tokenizer,
            id2label,
        })
    }

    /// Load model from specific file paths.
    pub fn from_files<P1, P2, P3>(
        model_path: P1,
        tokenizer_path: P2,
        config_path: Option<P3>,
    ) -> MlNerResult<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
    {
        let session = Session::builder()
            .map_err(|e| Error::ParseError(format!("Failed to create session: {}", e)))?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| Error::ParseError(format!("Failed to set optimization level: {}", e)))?
            .commit_from_file(model_path.as_ref())
            .map_err(|e| Error::ParseError(format!("Failed to load model: {}", e)))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path.as_ref())
            .map_err(|e| Error::ParseError(format!("Failed to load tokenizer: {}", e)))?;

        let id2label = if let Some(config) = config_path {
            Self::load_id2label(config.as_ref())?
        } else {
            Self::default_id2label()
        };

        Ok(Self {
            session: Mutex::new(session),
            tokenizer,
            id2label,
        })
    }

    /// Extract named entities from text.
    pub fn extract(&self, text: &str) -> MlNerResult<Vec<MlEntity>> {
        // Tokenize
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| Error::ParseError(format!("Tokenization failed: {}", e)))?;

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|&m| m as i64)
            .collect();

        let seq_len = input_ids.len();

        // Create tensors using ort::Tensor::from_array with (shape, data) tuple
        let input_ids_tensor = Tensor::from_array((vec![1i64, seq_len as i64], input_ids))
            .map_err(|e| Error::ParseError(format!("Failed to create input tensor: {}", e)))?;
        let attention_mask_tensor = Tensor::from_array((vec![1i64, seq_len as i64], attention_mask))
            .map_err(|e| Error::ParseError(format!("Failed to create attention mask tensor: {}", e)))?;

        // Lock session for inference
        let mut session = self
            .session
            .lock()
            .map_err(|e| Error::ParseError(format!("Failed to lock session: {}", e)))?;

        // Run inference using ort::inputs! with named tensors
        let outputs = session
            .run(ort::inputs! {
                "input_ids" => input_ids_tensor,
                "attention_mask" => attention_mask_tensor
            })
            .map_err(|e| Error::ParseError(format!("Inference failed: {}", e)))?;

        // Get logits output by name
        let logits_value = outputs
            .get("logits")
            .ok_or_else(|| Error::ParseError("No logits output found".to_string()))?;

        let (_shape, logits_data) = logits_value
            .try_extract_tensor::<f32>()
            .map_err(|e| Error::ParseError(format!("Failed to extract logits: {}", e)))?;

        // Process predictions
        let entities = self.decode_predictions(text, &encoding, logits_data)?;

        Ok(entities)
    }

    /// Extract entities and convert to standard Entity type.
    pub fn extract_entities(&self, text: &str) -> MlNerResult<Vec<Entity>> {
        let ml_entities = self.extract(text)?;
        Ok(ml_entities.into_iter().map(|e| e.to_entity()).collect())
    }

    fn decode_predictions(
        &self,
        text: &str,
        encoding: &tokenizers::Encoding,
        logits: &[f32],
    ) -> MlNerResult<Vec<MlEntity>> {
        let num_labels = self.id2label.len();

        let mut entities = Vec::new();
        let mut current_entity: Option<(String, String, f32, usize, usize)> = None;

        for (i, _token_idx) in encoding.get_ids().iter().enumerate() {
            // Skip special tokens
            if encoding.get_special_tokens_mask()[i] == 1 {
                // Finalize any current entity
                if let Some((label, ent_text, score, start, end)) = current_entity.take() {
                    entities.push(MlEntity {
                        text: ent_text,
                        label,
                        score,
                        start,
                        end,
                    });
                }
                continue;
            }

            // Get logits for this token
            let start_idx = i * num_labels;
            let end_idx = start_idx + num_labels;

            if end_idx > logits.len() {
                break;
            }

            let token_logits = &logits[start_idx..end_idx];

            // Softmax and get prediction
            let (pred_label_id, prob) = Self::softmax_argmax(token_logits);

            let label = self
                .id2label
                .get(&pred_label_id)
                .cloned()
                .unwrap_or_else(|| "O".to_string());

            // Get token offsets in original text
            let offsets = encoding.get_offsets()[i];
            let token_start = offsets.0;
            let token_end = offsets.1;

            // Skip "O" (Outside) labels
            if label == "O" {
                if let Some((lbl, txt, score, start, end)) = current_entity.take() {
                    entities.push(MlEntity {
                        text: txt,
                        label: lbl,
                        score,
                        start,
                        end,
                    });
                }
                continue;
            }

            // Handle BIO tagging
            let is_beginning = label.starts_with("B-");
            let entity_type = if is_beginning || label.starts_with("I-") {
                &label[2..]
            } else {
                &label
            };

            match &mut current_entity {
                Some((curr_label, curr_text, curr_score, curr_start, curr_end)) => {
                    let curr_type = if curr_label.starts_with("B-") || curr_label.starts_with("I-")
                    {
                        &curr_label[2..]
                    } else {
                        curr_label.as_str()
                    };

                    if is_beginning || entity_type != curr_type {
                        // Start new entity, save previous
                        entities.push(MlEntity {
                            text: curr_text.clone(),
                            label: curr_label.clone(),
                            score: *curr_score,
                            start: *curr_start,
                            end: *curr_end,
                        });

                        let token_text = &text[token_start..token_end];
                        current_entity = Some((
                            label.clone(),
                            token_text.to_string(),
                            prob,
                            token_start,
                            token_end,
                        ));
                    } else {
                        // Continue current entity
                        let token_text = &text[*curr_end..token_end];
                        curr_text.push_str(token_text);
                        *curr_end = token_end;
                        *curr_score = (*curr_score + prob) / 2.0; // Average confidence
                    }
                }
                None => {
                    let token_text = &text[token_start..token_end];
                    current_entity = Some((
                        label.clone(),
                        token_text.to_string(),
                        prob,
                        token_start,
                        token_end,
                    ));
                }
            }
        }

        // Don't forget the last entity
        if let Some((label, ent_text, score, start, end)) = current_entity {
            entities.push(MlEntity {
                text: ent_text,
                label,
                score,
                start,
                end,
            });
        }

        // Clean up entity text (remove ## subword markers, trim)
        for entity in &mut entities {
            entity.text = entity.text.replace("##", "").trim().to_string();
        }

        // Filter out empty entities
        entities.retain(|e| !e.text.is_empty());

        Ok(entities)
    }

    fn softmax_argmax(logits: &[f32]) -> (i64, f32) {
        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();

        let mut max_idx = 0;
        let mut max_prob = 0.0f32;

        for (i, &logit) in logits.iter().enumerate() {
            let prob = (logit - max_logit).exp() / exp_sum;
            if prob > max_prob {
                max_prob = prob;
                max_idx = i;
            }
        }

        (max_idx as i64, max_prob)
    }

    fn load_id2label(config_path: &Path) -> MlNerResult<HashMap<i64, String>> {
        let content = std::fs::read_to_string(config_path)
            .map_err(|e| Error::ParseError(format!("Failed to read config: {}", e)))?;

        let config: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| Error::ParseError(format!("Failed to parse config: {}", e)))?;

        let mut id2label = HashMap::new();

        if let Some(mapping) = config.get("id2label").and_then(|v| v.as_object()) {
            for (key, value) in mapping {
                if let (Ok(id), Some(label)) = (key.parse::<i64>(), value.as_str()) {
                    id2label.insert(id, label.to_string());
                }
            }
        }

        if id2label.is_empty() {
            return Ok(Self::default_id2label());
        }

        Ok(id2label)
    }

    fn default_id2label() -> HashMap<i64, String> {
        // CoNLL-2003 default labels
        let mut map = HashMap::new();
        map.insert(0, "O".to_string());
        map.insert(1, "B-PER".to_string());
        map.insert(2, "I-PER".to_string());
        map.insert(3, "B-ORG".to_string());
        map.insert(4, "I-ORG".to_string());
        map.insert(5, "B-LOC".to_string());
        map.insert(6, "I-LOC".to_string());
        map.insert(7, "B-MISC".to_string());
        map.insert(8, "I-MISC".to_string());
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_entity_to_entity() {
        let ml_entity = MlEntity {
            text: "Paris".to_string(),
            label: "B-LOC".to_string(),
            score: 0.95,
            start: 0,
            end: 5,
        };

        let entity = ml_entity.to_entity();
        assert!(matches!(entity.entity_type, EntityType::Location));
        assert_eq!(entity.text, "Paris");
    }

    #[test]
    fn test_default_id2label() {
        let labels = MlNerModel::default_id2label();
        assert_eq!(labels.get(&0), Some(&"O".to_string()));
        assert_eq!(labels.get(&1), Some(&"B-PER".to_string()));
        assert_eq!(labels.get(&5), Some(&"B-LOC".to_string()));
    }

    #[test]
    fn test_softmax_argmax() {
        let logits = vec![1.0, 2.0, 3.0, 0.5];
        let (idx, prob) = MlNerModel::softmax_argmax(&logits);
        assert_eq!(idx, 2); // index of 3.0
        assert!(prob > 0.5); // should be highest probability
    }
}
