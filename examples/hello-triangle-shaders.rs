use hassle_rs::{validate_dxil, DxcPack};

fn main() {
    use hassle_rs::compile_hlsl;

    let code = include_str!("hello-triangle-shaders.hlsl");

    let ir = compile_hlsl("hello-triangle-shaders.hlsl", code, Some("VSMain"), "vs_6_0", &[], &[]).unwrap();
    println!("{ir:?}");
    let validated = validate_dxil(&ir).unwrap();
    println!("{validated:?}");

    let dxc = DxcPack::create().unwrap();
    let blob = dxc.compile_validate("hello-triangle-shaders.hlsl", code, Some("VSMain"), "vs_6_0", &[], &[]).unwrap();
    println!("{:?}", blob);
}
