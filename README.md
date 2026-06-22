# Network Protocol 📡

Custom binary protocol with multiplexing and encryption.

## Protocol Features

- **Framing**: Length-prefixed messages
- **Multiplexing**: Concurrent streams
- **Compression**: LZ4 per-frame
- **Encryption**: AES-256-GCM

## Performance

| Metric | Value |
|--------|-------|
| Throughput | 20 Gbps |
| Latency | 50μs |
| Frame overhead | 9 bytes |

## Quick Start

```rust
let codec = ProtocolCodec::new(Encryption::AES256);
let frame = codec.encode(b"hello")?;
let decoded = codec.decode(&frame)?;
```

## License

Apache 2.0