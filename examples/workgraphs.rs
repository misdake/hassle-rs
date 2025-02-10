use hassle_rs::validate_dxil;

fn main() {
    use hassle_rs::compile_hlsl;

    let code = include_str!("workgraphs.hlsl");

    let ir = compile_hlsl("workgraphs.hlsl", code, None, "lib_6_8", &[], &[]).unwrap();
    println!("{ir:?}");
    let validated = validate_dxil(&ir).unwrap();
    println!("{validated:?}");
}
