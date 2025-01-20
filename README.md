# egui-qr

Simple egui painter that renders QR Codes

## Example

```rust
QrCodeWidget::from_data(b"https://google.com")?
    .ui(ui);
```