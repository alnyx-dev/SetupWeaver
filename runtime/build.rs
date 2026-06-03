// runtime/build.rs
fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("fluent".into());
    slint_build::compile_with_config("ui.slint", config).expect("failed to compile Slint UI");
}
