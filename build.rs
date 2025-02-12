fn main() {
    println!("cargo::rustc-check-cfg=cfg(is_v_7_1)");
    println!("cargo::rustc-check-cfg=cfg(is_v_7_0)");
    println!("cargo::rustc-check-cfg=cfg(is_v_6_1)");

    if cfg!(feature = "v7_1") {
        println!("cargo:rustc-cfg=is_v_7_1");
    } else if cfg!(feature = "v7_0") {
        println!("cargo:rustc-cfg=is_v_7_0");
    } else if cfg!(feature = "v6_1") {
        println!("cargo:rustc-cfg=is_v_6_1");
    } else {
        println!("cargo:rustc-cfg=is_v_7_1");
    }
}