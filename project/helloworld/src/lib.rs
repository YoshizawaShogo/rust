/// cargo doc の練習
/// 
/// 対象の関数の上に記述する必要あり。
/// 
/// また、書き方がMarkdown形式であり、空行とインデントが必要。
/// 
///     helloworld::helloworld();
/// 
/// また、cargo test によるテストも可能。
/// 
///     assert!(true);

pub fn helloworld() {
    // その他のクレートから実行できるようにpubを付けた。
    println!("Hello World.");
}

#[test]
fn test_helloworld() {
    // #[test] 属性を付与することでcargo test実行時に呼び出される。
    // 同一クレート、同一モジュール内の関数を直接実行できる。
    helloworld();
}