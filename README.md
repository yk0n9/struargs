# struargs

A macro that builds a structure into a Command parameter list

## Example

```rust
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
```

it expand to (all field must impl Display)

```rust
    impl ::struargs::Args for StructArg {
        fn args(&self) -> Vec<String> {
            let mut args = ::alloc::vec::Vec::new();
            if let Some(ref value) = self.size {
                args.extend(["--size".to_string(), value.to_string()]);
            }
            if let Some(ref value) = self.name {
                args.extend(["--name".to_string(), value.to_string()]);
            }
            if let Some(ref value) = self.ty {
                args.extend(["--type".to_string(), value.to_string()]);
            }
            if self.one.is_some() {
                args.extend(["--one".to_string()]);
            }
            args.extend(["--num".to_string(), self.num.to_string()]);
            args.extend(["-e".to_string(), self.enable.to_string()]);
            args.extend(["--a_b".to_string(), self.a_b.to_string()]);
            args
        }
        fn env_args(&self) -> ::std::collections::HashMap<String, String> {
            let mut env_vars = ::std::collections::HashMap::new();
            if let Some(ref value) = self.size {
                env_vars.insert("SIZE".to_string(), value.to_string());
            }
            if let Some(ref value) = self.name {
                env_vars.insert("NAME".to_string(), value.to_string());
            }
            if let Some(ref value) = self.ty {
                env_vars.insert("TYPE".to_string(), value.to_string());
            }
            env_vars.insert("NUM".to_string(), self.num.to_string());
            env_vars.insert("ENABLE".to_string(), self.enable.to_string());
            env_vars.insert("A_B".to_string(), self.a_b.to_string());
            env_vars
        }
    }
```
### Args

- rename (custom)
- no_value
- short (custom)
