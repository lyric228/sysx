# sysx (legacy) → Migrate to [sysz](https://github.com/lyric228/sysz)

> **Project Archived**  
> This repository is no longer maintained. All development has moved to its next-generation successor:  
> **[sysz - Next-generation Rust System Utilities](https://github.com/lyric228/sysz)**

## Why the transition?

- 🚀 Full rewrite with improved error handling
- 💡 New features: async support, enhanced modules
- 📦 Better ergonomics and performance
- 🔄 Continuously maintained

## Migration Guide

1. Update your `Cargo.toml`:

```toml
[dependencies]
sysz = "0.2"  # Replace sysx dependency
```

2. Update imports (example):

```diff rust
- use sysx::io::cmd::run;
- use sysx::math::bin;
+ use sysz::io::cmd::run;
+ use sysz::math::bin;
```

3. Explore new features:

```rust
use sysz::time::sleep;
use sysz::net::ipv4;

async fn main() {
    sleep::sleep(100).await; // Async sleep
    ipv4::validate("192.168.0.1"); // Enhanced networking
}
```

## Final Notice

All new features and bug fixes will be implemented in `sysz`.

✨ **Join the new era:** [github.com/lyric228/sysz](https://github.com/lyric228/sysz)
