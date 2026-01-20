//! Demo: TCS with nested variable-length arrays
//!
//! Schema structure:
//!   Canvas (message)
//!     └── Layer[] (struct with String + Brush[])
//!           └── Brush[] (struct with Type + Color[])
//!                 └── Color[] (struct with RGBA bytes)

mod generated;

use generated::example::{Brush, Canvas, Color, Layer, Type};

fn main() {
    // Create some colors
    let red = Color { red: 255, green: 0, blue: 0, alpha: 255 };
    let green = Color { red: 0, green: 255, blue: 0, alpha: 255 };
    let blue = Color { red: 0, green: 0, blue: 255, alpha: 255 };
    let yellow = Color { red: 255, green: 255, blue: 0, alpha: 200 };
    let white = Color { red: 255, green: 255, blue: 255, alpha: 255 };

    // Create brushes with different shapes and color gradients
    let round_brush = Brush {
        type_: Type::Round,
        colors: vec![red.clone(), yellow.clone()], // red-to-yellow gradient
    };

    let flat_brush = Brush {
        type_: Type::Flat,
        colors: vec![blue.clone(), green.clone(), white.clone()], // 3-color gradient
    };

    let pointed_brush = Brush {
        type_: Type::Pointed,
        colors: vec![white.clone()], // solid white
    };

    // Create layers with multiple brushes
    let background = Layer {
        name: "Background".to_string(),
        brushes: vec![flat_brush],
    };

    let foreground = Layer {
        name: "Foreground".to_string(),
        brushes: vec![round_brush, pointed_brush],
    };

    // Create canvas with nested structure
    let canvas = Canvas {
        client_id: Some(42),
        width: Some(1920),
        height: Some(1080),
        layers: Some(vec![background, foreground]),
    };

    println!("Original Canvas:");
    println!("{:#?}", canvas);

    // Serialize to bytes
    let bytes = canvas.to_bytes();
    println!("\nSerialized: {} bytes", bytes.len());
    println!("Hex: {:02x?}", &bytes[..bytes.len().min(64)]);
    if bytes.len() > 64 {
        println!("    ... ({} more bytes)", bytes.len() - 64);
    }

    // Deserialize back
    let decoded = Canvas::from_bytes(&bytes).expect("deserialization failed");

    // Verify round-trip
    assert_eq!(canvas, decoded);
    println!("\n✓ Round-trip successful!");
    println!("  Nested structure: Canvas -> Layer[] -> Brush[] -> Color[]");
}
