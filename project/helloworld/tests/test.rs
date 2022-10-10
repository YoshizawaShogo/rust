#[test]
fn tsets_helloworld() {
    // helloworldクレート(このファイルがあるクレート)のhelloworld()関数を実行。
    // testsディレクトリでの同一クレートの関数呼び出しは、他のクレートからの呼び出しのように記述する。
    helloworld::helloworld();
}