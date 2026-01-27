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
        if let Some(_) = self.one {
            args.extend(["--one".to_string()]);
        }
        args.extend(["--num".to_string(), self.num.to_string()]);
        if let Some(ref value) = self.enable {
            args.extend(["-e".to_string(), value.to_string()]);
        }
        args
    }
}
```
### Args

- rename (custom)
- no_value
- short (custom)
