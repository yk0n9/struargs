#[test]
fn test() {
    use struargs::Args;

    #[derive(Debug, Args)]
    struct StructArg {
        size: Option<i32>,
        name: Option<String>,
        #[args(rename = "type")]
        ty: Option<String>,
        #[args(no_value)]
        one: Option<()>,
        num: f32,
    }

    let s = StructArg {
        size: None,
        name: Some("123".to_string()),
        ty: Some("Arg".to_string()),
        one: Some(()),
        num: 100.1,
    };

    let args = s.args().join(" ");

    assert_eq!(args, "--name 123 --type Arg --one --num 100.1");
}
