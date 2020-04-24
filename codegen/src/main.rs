mod sqlstate;
mod type_gen;

fn main() -> std::io::Result<()> {
    type_gen::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/ty/gen.rs"))?;
    sqlstate::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/state/gen.rs"))
}
