pub trait Args {
    fn args(&self) -> Vec<String>;
}

pub use struargs_derive::Args;
