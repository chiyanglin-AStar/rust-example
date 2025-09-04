# rust-example

### Rust 專案範例

我們將建立一個簡單的 Rust 專案，它包含一個 `main.rs` 和一個獨立的模組 `utils.rs`。

**專案結構:**

```
rust-example/
├── src/
│   ├── main.rs
│   └── utils.rs
├── Cargo.toml
```

**`Cargo.toml`:**
```toml
[package]
name = "rust-example"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**`src/main.rs`:**
```rust
mod utils; // 宣告使用 utils 模組

fn main() {
    let message = "Hello from main!";
    println!("{}", message);

    utils::print_current_time(); // 呼叫 utils 模組中的函數
    utils::print_message("This message came from main via utils.");
}
```

**`src/utils.rs`:**
```rust
use chrono::{Local, Timelike}; // 引入 chrono 庫用於時間
// 注意：如果需要 chrono，你需要在 Cargo.toml 的 [dependencies] 中添加 `chrono = "0.4"`
// 但為了簡化 Yocto 範例，我們這裡暫時不引入外部依賴。
// 如果要保持這個例子簡單，你可以將時間部分替換成一個簡單的字串。
// 例如：
pub fn print_current_time() {
    println!("Current time (placeholder): XX:XX:XX");
}

pub fn print_message(msg: &str) {
    println!("Utils module received: {}", msg);
}


// 如果要使用真實的時間，請確保 Cargo.toml 中有 [dependencies] chrono = "0.4"
/*
// 實際使用 chrono 的版本：
pub fn print_current_time() {
    let now = Local::now();
    println!("Current time from utils: {}:{}:{}", now.hour(), now.minute(), now.second());
}

pub fn print_message(msg: &str) {
    println!("Utils module received: {}", msg);
}
*/
```

**關於 `chrono` 注意事項:**
為了讓 Yocto recipe 盡可能簡單，我們將 `src/utils.rs` 中的 `chrono` 相關程式碼註釋掉或替換為簡單版本。如果你的 Rust 專案需要外部 `crates.io` 依賴，Yocto 的 `rust` class 會自動處理這些依賴，但通常會要求這些依賴在 `meta-rust` 或其他層中已定義為 Yocto recipe。對於這個基本範例，我們將避免外部依賴。
