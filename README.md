# egui-qr

Simple egui painter that renders QR Codes

## Example

```rust
QrCodeWidget::from_data(b"https://google.com")?
    .ui(ui);

// a fixed size, with a wider quiet zone for phone cameras
QrCodeWidget::from_data(b"bitcoin:BC1Q...")?
    .size(240.0)
    .quiet_zone(3.0)
    .ui(ui);
```

Version 0.2 needs egui 0.35. Use 0.1 for egui 0.33.