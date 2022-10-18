// 文字列入力を必要とする物は後半に配置し、不要ならばCtrl + Cで終了する。


fn main() {
    // リテラル文字列出力
    println!("Hello world.");

    // i32 変数宣言, 変数の出力
    let a = 10; // 型推論
    println!("{}", a);
    assert_eq!(type_of(a), "i32"); // 等しくなければ、プログラム終了

    // i64 明示的な変数宣言
    let a:i64 = 10;
    assert_eq!(type_of(a), "i64");

    // ただの文字列は&str型
    // &str型とString型は別
    let _s: &str = "&str";
    let _s: String = String::from("String");

    // 配列, ベクトル
    let _array = [0, 1, 2, 3, 4];
    let _vec = vec![0, 1, 2, 3, 4];
    let _vec2 = _array.to_vec();

    // 三項演算子
    let a = if true { 10 } else { 2 };
    assert_eq!(a, 10);

    // swap方法
    let mut x = 20;
    let mut y = 3;
    (x, y) = (y, x);
    assert_eq!(x, 3);
    assert_eq!(y, 20);

    // 文字列分割
    let split: Vec<&str> = "some string 123 ffd".split(" ").collect();
    let vector: Vec<&str> = vec!["some", "string", "123", "ffd"];
    assert_eq!(split, vector);
    println!("{:?}", vector); // debug出力

    // 日本語を変数名とすることもできる
    let 空気 = "air";
    assert_eq!(空気, "air");

    // 文字列入力
    let mut buf = String::new();
    println!("文字列を入力してください");
    let buf_size = std::io::stdin().read_line(&mut buf).expect("何らかの文字が入力されることを期待");
    println!("size {} の文字列 {} が入力されました", buf_size, buf.trim());
}

// 型取得関数, 参考URL: http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}