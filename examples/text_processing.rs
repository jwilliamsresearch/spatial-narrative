//! Text Processing Example
//!
//! This example demonstrates how to use the text processing capabilities
//! of spatial-narrative to extract geographic and semantic information from text.
//!
//! Run with: cargo run --example text_processing

use spatial_narrative::parser::{BuiltinGazetteer, GeoParser, MentionType};
use spatial_narrative::text::{EntityType, KeywordExtractor, TextAnalyzer};

fn main() {
    println!("=== Spatial Narrative Text Processing Demo ===\n");

    // Sample narrative text
    let narrative = r#"
        Dr. Sarah Chen, a climate scientist at MIT, presented her findings 
        at the Global Climate Summit in Paris on March 15, 2024. Her research 
        focused on rising sea levels affecting coastal cities like New York, 
        Miami, and Tokyo. The study, funded by the National Science Foundation 
        with a $2.5 million grant, analyzed data from coordinates 40.7128°N, 
        74.0060°W to 35.6762°N, 139.6503°E. Representatives from NASA and the 
        World Health Organization attended the presentation.
    "#;

    // =========================================
    // Part 1: Geoparsing - Extract Locations
    // =========================================
    println!("--- GEOPARSING ---\n");

    let gazetteer = BuiltinGazetteer::new();
    let parser = GeoParser::with_gazetteer(Box::new(gazetteer));

    let mentions = parser.extract(narrative);

    println!("Found {} location mentions:\n", mentions.len());

    for mention in &mentions {
        let type_str = match mention.mention_type {
            MentionType::DecimalDegrees => "Decimal Degrees",
            MentionType::DegreesWithSymbols => "Degrees w/ Symbols",
            MentionType::DMS => "DMS",
            MentionType::PlaceName => "Place Name",
            MentionType::Address => "Address",
        };

        print!("  [{:<18}] \"{}\"", type_str, mention.text);

        if let Some(ref loc) = mention.location {
            print!(" -> ({:.4}, {:.4})", loc.lat, loc.lon);
        }

        println!(" [confidence: {:.2}]", mention.confidence);
    }

    // =========================================
    // Part 2: Named Entity Recognition
    // =========================================
    println!("\n--- NAMED ENTITY RECOGNITION ---\n");

    let analyzer = TextAnalyzer::new();
    let entities = analyzer.entities(narrative);

    // Group entities by type
    let mut persons = Vec::new();
    let mut orgs = Vec::new();
    let mut dates = Vec::new();
    let mut numerics = Vec::new();
    let mut locations = Vec::new();

    for entity in entities {
        match entity.entity_type {
            EntityType::Person => persons.push(entity),
            EntityType::Organization => orgs.push(entity),
            EntityType::DateTime => dates.push(entity),
            EntityType::Numeric => numerics.push(entity),
            EntityType::Location => locations.push(entity),
            _ => {},
        }
    }

    println!("People ({}):", persons.len());
    for p in &persons {
        println!("  - {} [confidence: {:.2}]", p.text, p.confidence);
    }

    println!("\nOrganizations ({}):", orgs.len());
    for o in &orgs {
        println!("  - {} [confidence: {:.2}]", o.text, o.confidence);
    }

    println!("\nDates ({}):", dates.len());
    for d in &dates {
        println!("  - {} [confidence: {:.2}]", d.text, d.confidence);
    }

    println!("\nNumeric Values ({}):", numerics.len());
    for n in &numerics {
        println!("  - {} [confidence: {:.2}]", n.text, n.confidence);
    }

    println!("\nLocations ({}):", locations.len());
    for l in &locations {
        println!("  - {} [confidence: {:.2}]", l.text, l.confidence);
    }

    // =========================================
    // Part 3: Keyword Extraction
    // =========================================
    println!("\n--- KEYWORD EXTRACTION ---\n");

    let extractor = KeywordExtractor::new();

    // Extract top keywords
    let keywords = extractor.extract(narrative, 10);
    println!("Top 10 Keywords:");
    for (i, kw) in keywords.iter().enumerate() {
        println!(
            "  {}. {} (score: {:.3}, freq: {})",
            i + 1,
            kw.text,
            kw.score,
            kw.frequency
        );
    }

    // Extract with phrases (bigrams/trigrams included by default)
    let extractor_phrases = KeywordExtractor::new().max_phrase_length(2);
    let phrases = extractor_phrases.extract(narrative, 5);
    println!("\nTop 5 Terms (with bigrams):");
    for (i, phrase) in phrases.iter().enumerate() {
        println!("  {}. {} (score: {:.3})", i + 1, phrase.text, phrase.score);
    }

    // =========================================
    // Part 4: Text Utilities
    // =========================================
    println!("\n--- TEXT UTILITIES ---\n");

    let sample = "First sentence about climate. Second about cities! Third about data?";
    let sentences = analyzer.sentences(sample);
    println!("Sentences ({}):", sentences.len());
    for (i, s) in sentences.iter().enumerate() {
        println!("  {}. {}", i + 1, s);
    }

    let tokens = analyzer.tokenize(sample);
    println!("\nWord count: {}", tokens.len());

    // =========================================
    // Part 5: Geocoding
    // =========================================
    println!("\n--- GEOCODING ---\n");

    let cities = ["London", "Tokyo", "NYC", "Sydney", "NonexistentCity"];

    for city in cities {
        match parser.geocode(city) {
            Some(loc) => println!("  {} -> ({:.4}, {:.4})", city, loc.lat, loc.lon),
            None => println!("  {} -> Not found", city),
        }
    }

    println!("\n=== Demo Complete ===");
}
