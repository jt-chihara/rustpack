[![Unit Tests](https://github.com/jt-chihara/rustpack/actions/workflows/rust.yml/badge.svg)](https://github.com/jt-chihara/rustpack/actions/workflows/rust.yml)

# rustpack

Rust 2D rectangle packing library inspired by [rectpack](https://github.com/secnot/rectpack).

## Features

- Multiple packing algorithms: Bottom Left, MaxRects, Skyline, Guillotine
- Algorithm variants (BSSF, BAF, BLSF, etc.)
- Rectangle rotation support
- Multiple bin packing
- High performance with memory efficiency

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustpack = "0.1"
```

## Usage

### Basic Usage

```rust
use rustpack::*;

fn main() {
    let mut packer = Packer::new();
    
    // Add rectangles to pack
    packer.add_rect(Rectangle { width: 10, height: 15 });
    packer.add_rect(Rectangle { width: 8, height: 12 });
    packer.add_rect(Rectangle { width: 6, height: 8 });
    
    // Add bins (containers)
    packer.add_bin(20, 30);
    
    // Pack rectangles
    packer.pack();
    
    // Get results
    for rect in packer.rect_list() {
        println!("Rectangle at ({}, {}) size {}x{} in bin {}", 
                 rect.x, rect.y, rect.width, rect.height, rect.bin_id);
    }
}
```

### Algorithm Selection

```rust
use rustpack::*;

let mut packer = Packer::new();

// Choose packing algorithm
packer.set_pack_algo(PackAlgorithm::MaxRects);        // Default MaxRects
packer.set_pack_algo(PackAlgorithm::MaxRectsBssf);    // Best Short Side Fit
packer.set_pack_algo(PackAlgorithm::MaxRectsBaf);     // Best Area Fit  
packer.set_pack_algo(PackAlgorithm::MaxRectsBlsf);    // Best Long Side Fit
packer.set_pack_algo(PackAlgorithm::Skyline);         // Skyline algorithm
packer.set_pack_algo(PackAlgorithm::SkylineBl);       // Skyline Bottom Left
packer.set_pack_algo(PackAlgorithm::Guillotine);      // Guillotine algorithm
packer.set_pack_algo(PackAlgorithm::GuillotineBssfSas); // Guillotine BSSF-SAS
packer.set_pack_algo(PackAlgorithm::BottomLeft);      // Bottom Left algorithm
```

### Rectangle Rotation

```rust
use rustpack::*;

let mut packer = Packer::new();
packer.enable_rotation();  // Allow 90-degree rotation

packer.add_rect(Rectangle { width: 20, height: 10 });
packer.add_bin(15, 25);    // Rectangle will be rotated to fit

packer.pack();

let rects = packer.rect_list();
if !rects.is_empty() && rects[0].rotated {
    println!("Rectangle was rotated!");
}
```

### Multiple Bins

```rust
use rustpack::*;

let mut packer = Packer::new();

// Add many rectangles
for i in 0..10 {
    packer.add_rect(Rectangle { width: 8, height: 8 });
}

// Add multiple bins
packer.add_bin(20, 20);
packer.add_bin(20, 20);
packer.add_bin(20, 20);

packer.pack();

// Rectangles will be distributed across bins
for rect in packer.rect_list() {
    println!("Rectangle in bin {}", rect.bin_id);
}
```

## API Reference

### Packer

- `Packer::new()` - Create new packer
- `add_rect(Rectangle)` - Add rectangle to pack
- `add_bin(width, height)` - Add container bin
- `set_pack_algo(PackAlgorithm)` - Set packing algorithm
- `enable_rotation()` - Allow rectangle rotation
- `disable_rotation()` - Disable rectangle rotation
- `pack()` - Execute packing
- `rect_list()` - Get packed rectangle positions

### Rectangle

```rust
Rectangle { width: u32, height: u32 }
```

### PackedRect

```rust
PackedRect {
    x: u32,          // Position X
    y: u32,          // Position Y  
    width: u32,      // Final width (after rotation)
    height: u32,     // Final height (after rotation)
    rotated: bool,   // Was rotated?
    bin_id: usize,   // Which bin it's in
}
```

## Algorithms

### Bottom Left
Places rectangles at the bottom-left most position.

### MaxRects
Maintains a list of free rectangles and uses various heuristics:
- **MaxRects**: Default variant
- **MaxRectsBssf**: Best Short Side Fit
- **MaxRectsBaf**: Best Area Fit  
- **MaxRectsBlsf**: Best Long Side Fit

### Skyline
Maintains a skyline and places rectangles at the lowest position.
- **Skyline**: Default variant
- **SkylineBl**: Bottom Left variant

### Guillotine
Uses guillotine cuts to split free space.
- **Guillotine**: Default variant
- **GuillotineBssfSas**: Best Short Side Fit with Shorter Axis Split

## Performance

All algorithms are optimized for speed and memory usage. For performance-critical applications, MaxRects variants typically provide the best balance of packing efficiency and speed.

## Testing

```bash
cargo test
```

## License

MIT
