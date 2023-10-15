# cnote
cli rust protocol analyzer

## CLI
Making it a cli first then a gui

lot of refactoring coming up too after this refactoring.

## How it works

### base frame
```rust
#[derive(Default)]
pub struct EthernetFrame {
    pub id: i32,
    pub header: EthernetHeader,
    pub payload: Option<Box<dyn Layer>>,
}
```
Layer is a trait object that gets implemented further down the line.

### Layers
Every packet implements the layer trait, layer implements the ```Send``` marker

### Sniffers
Sniffers are the packet capture logic. 

This is just a rough outline for anyone who reads this/myself so i know what im doing.


Each packet that implements layer handles the logic for the layer it encapsulates.

### Milestones
1. Establish a general framework for the backend that is a reasonable approach
🔵🔵🔵🔵🔵⬜⬜⬜⬜⬜ 50%
2. Set up basic gui - ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ 0%
3. Figure out how to handle 802.11 frame - ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ 0%