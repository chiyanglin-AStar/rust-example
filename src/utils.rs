//use chrono::{Local, Timelike}; // 引入 chrono 庫用於時間
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
