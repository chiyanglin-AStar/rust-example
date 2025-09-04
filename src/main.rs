mod utils; // 宣告使用 utils 模組

fn main() {
    let message = "Hello from main!";
    println!("{}", message);

    utils::print_current_time(); // 呼叫 utils 模組中的函數
    utils::print_message("This message came from main via utils.");
}
