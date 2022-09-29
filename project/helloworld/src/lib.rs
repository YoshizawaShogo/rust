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