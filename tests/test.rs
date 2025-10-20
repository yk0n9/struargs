#[test]
fn test() {
    use struargs::Args;

    #[derive(Debug, Args)]
    struct StructArg {
        size: Option<i32>,
        name: Option<String>,
        #[args(rename = "type")]
        ty: Option<String>,
        num: f32,
    }

    let s = StructArg {
        size: None,
        name: Some("123".to_string()),
        ty: Some("Arg".to_string()),
        num: 100.1,
    };

    assert_eq!(
        s.args(),
        vec![
            "--name".to_string(),
            123.to_string(),
            "--type".to_string(),
            "Arg".to_string(),
            "--num".to_string(),
            "100.1".to_string(),
        ]
    );
}
