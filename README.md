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
        if let Some(_) = self.one {
            args.extend(["--one".to_string()]);
        }
        args.extend(["--num".to_string(), self.num.to_string()]);
        args
    }
}
```
### Args

- rename (custom)
- no_value
