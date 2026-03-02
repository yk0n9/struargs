pub use struargs_derive::Args;

pub trait Args {
    fn args(&self) -> Vec<String>;

    fn env_args(&self) -> std::collections::HashMap<String, String>;
}
