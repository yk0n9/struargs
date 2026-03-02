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
        a_b: bool,
    }

    let s = StructArg {
        size: None,
        name: Some("Ykong".to_string()),
        ty: Some("Arg".to_string()),
        one: Some(()),
        num: 100.1,
        enable: true,
        a_b: false,
    };

    let args = s.args().join(" ");

    assert_eq!(
        args,
        "--name Ykong --type Arg --one --num 100.1 -e true --a_b false"
    );

    let mut env_args = s
        .env_args()
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>();

    env_args.sort();

    assert_eq!(
        env_args,
        vec![
            "A_B=false",
            "ENABLE=true",
            "NAME=Ykong",
            "NUM=100.1",
            "TYPE=Arg",
        ],
    );
}
