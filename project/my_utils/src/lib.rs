// 型取得関数, 参考URL: http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
pub fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}