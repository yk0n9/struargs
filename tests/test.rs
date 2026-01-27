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
        #[args(short = "e")]
        enable: bool,
    }

    let s = StructArg {
        size: None,
        name: Some("Ykong".to_string()),
        ty: Some("Arg".to_string()),
        one: Some(()),
        num: 100.1,
        enable: true,
    };

    let args = s.args().join(" ");

    assert_eq!(args, "--name Ykong --type Arg --one --num 100.1 -e true");
}
