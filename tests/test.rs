use struargs::Args;

#[test]
fn test() {
    #[derive(Debug, Args)]
    struct StructArg {
        size: Option<i32>,
        name: Option<String>,
    }

    let s = StructArg {
        size: Some(1),
        name: Some("123".to_string()),
    };

    assert_eq!(
        s.args(),
        vec![
            "--size".to_string(),
            1.to_string(),
            "--name".to_string(),
            123.to_string()
        ]
    );
}
