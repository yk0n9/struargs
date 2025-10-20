# struargs

A macro that builds a structure into a Command parameter list

## Example

```rust
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
```

it expand to (all field must impl Display)

```rust
impl ::struargs::Args for StructArg {
    fn args(&self) -> Vec<String> {
        let mut args = ::alloc::vec::Vec::new();
        if let Some(ref arg) = self.size {
            args.extend(["--size".to_string(), arg.to_string()]);
        }
        if let Some(ref arg) = self.name {
            args.extend(["--name".to_string(), arg.to_string()]);
        }
        if let Some(ref arg) = self.ty {
            args.extend(["--type".to_string(), arg.to_string()]);
        }
        args.extend(["--num".to_string(), self.num.to_string()]);
        args
    }
}
```
