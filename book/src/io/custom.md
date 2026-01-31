# Custom Formats

Implement the `Format` trait to create custom import/export formats.

## The Format Trait

```rust
use spatial_narrative::io::Format;
use spatial_narrative::core::Narrative;
use spatial_narrative::error::Result;
use std::io::{Read, Write};

pub trait Format {
    /// Import a narrative from a reader
    fn import<R: Read>(&self, reader: &mut R) -> Result<Narrative>;
    
    /// Export a narrative to a writer
    fn export<W: Write>(&self, narrative: &Narrative, writer: &mut W) -> Result<()>;
    
    /// Import from a string (has default implementation)
    fn import_str(&self, s: &str) -> Result<Narrative> {
        self.import(&mut s.as_bytes())
    }
    
    /// Export to a string (has default implementation)
    fn export_str(&self, narrative: &Narrative) -> Result<String> {
        let mut buf = Vec::new();
        self.export(narrative, &mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
```

## Example: XML Format

```rust
use spatial_narrative::io::Format;
use spatial_narrative::core::{Narrative, NarrativeBuilder, Event, Location, Timestamp};
use spatial_narrative::error::{Error, Result};
use std::io::{Read, Write};

pub struct XmlFormat;

impl XmlFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for XmlFormat {
    fn import<R: Read>(&self, reader: &mut R) -> Result<Narrative> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        // Parse XML (using quick-xml or similar)
        // This is a simplified example
        let mut builder = NarrativeBuilder::new();
        
        // ... parse events from XML ...
        
        Ok(builder.build())
    }
    
    fn export<W: Write>(&self, narrative: &Narrative, writer: &mut W) -> Result<()> {
        writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        writeln!(writer, "<narrative>")?;
        
        if let Some(title) = &narrative.title {
            writeln!(writer, "  <title>{}</title>", escape_xml(title))?;
        }
        
        writeln!(writer, "  <events>")?;
        for event in &narrative.events {
            writeln!(writer, "    <event>")?;
            writeln!(writer, "      <lat>{}</lat>", event.location.lat)?;
            writeln!(writer, "      <lon>{}</lon>", event.location.lon)?;
            writeln!(writer, "      <timestamp>{}</timestamp>", 
                event.timestamp.to_rfc3339())?;
            writeln!(writer, "      <text>{}</text>", escape_xml(&event.text))?;
            writeln!(writer, "    </event>")?;
        }
        writeln!(writer, "  </events>")?;
        writeln!(writer, "</narrative>")?;
        
        Ok(())
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}
```

## Example: KML Format

For Google Earth compatibility:

```rust
pub struct KmlFormat;

impl Format for KmlFormat {
    fn export<W: Write>(&self, narrative: &Narrative, writer: &mut W) -> Result<()> {
        writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        writeln!(writer, r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#)?;
        writeln!(writer, "<Document>")?;
        
        if let Some(title) = &narrative.title {
            writeln!(writer, "  <name>{}</name>", title)?;
        }
        
        for event in &narrative.events {
            writeln!(writer, "  <Placemark>")?;
            writeln!(writer, "    <name>{}</name>", escape_xml(&event.text))?;
            writeln!(writer, "    <TimeStamp>")?;
            writeln!(writer, "      <when>{}</when>", event.timestamp.to_rfc3339())?;
            writeln!(writer, "    </TimeStamp>")?;
            writeln!(writer, "    <Point>")?;
            writeln!(writer, "      <coordinates>{},{}</coordinates>",
                event.location.lon, event.location.lat)?;
            writeln!(writer, "    </Point>")?;
            writeln!(writer, "  </Placemark>")?;
        }
        
        writeln!(writer, "</Document>")?;
        writeln!(writer, "</kml>")?;
        Ok(())
    }
    
    fn import<R: Read>(&self, _reader: &mut R) -> Result<Narrative> {
        // KML import implementation
        todo!("KML import not yet implemented")
    }
}
```

## Using Custom Formats

```rust
let narrative = create_narrative();

// Use custom XML format
let xml_format = XmlFormat::new();
let xml = xml_format.export_str(&narrative)?;

// Use custom KML format
let kml_format = KmlFormat::new();
let kml = kml_format.export_str(&narrative)?;

// Save to file
let mut file = File::create("narrative.kml")?;
kml_format.export(&narrative, &mut file)?;
```

## Best Practices

1. **Handle errors gracefully**: Return `Result` with descriptive errors
2. **Use buffered I/O**: Wrap readers/writers in `BufReader`/`BufWriter`
3. **Support round-trips**: Ensure `import(export(n)) == n` where possible
4. **Document limitations**: Note what metadata is lost in conversion
5. **Validate on import**: Check for required fields and valid values

## Error Handling

Use the library's error types:

```rust
use spatial_narrative::error::{Error, Result};

impl Format for MyFormat {
    fn import<R: Read>(&self, reader: &mut R) -> Result<Narrative> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        if content.is_empty() {
            return Err(Error::Parse("Empty input".to_string()));
        }
        
        // Parse and validate...
        
        Ok(narrative)
    }
}
```
