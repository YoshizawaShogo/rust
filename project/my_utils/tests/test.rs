use my_utils::type_of;

#[test]
fn test_type_of(){
    // i32 暗黙的な変数宣言
    let a = 10;
    assert_eq!(type_of(a), "i32");
    // i32 明示的な変数宣言
    let a: i32 = 10;
    assert_eq!(type_of(a), "i32");
    // i64 明示的な変数宣言
    let a: i64 = 10;
    assert_eq!(type_of(a), "i64");
}

