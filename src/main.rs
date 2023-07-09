use anyhow::Result;

pub mod common;
pub mod recursion;
#[cfg(test)]
pub mod tests;

fn main() -> Result<()> {
    recursion::recursion(2)
}
