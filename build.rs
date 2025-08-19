fn main() {
    println!("cargo::rustc-check-cfg=cfg(is_v_7_2)");
    println!("cargo::rustc-check-cfg=cfg(is_v_7_1)");
    println!("cargo::rustc-check-cfg=cfg(is_v_7_0)");
    println!("cargo::rustc-check-cfg=cfg(is_v_6_1)");

    println!("cargo::rustc-check-cfg=cfg(is_v_7_2_or_newer)");
    println!("cargo::rustc-check-cfg=cfg(is_v_7_1_or_newer)");
    println!("cargo::rustc-check-cfg=cfg(is_v_7_0_or_newer)");
    println!("cargo::rustc-check-cfg=cfg(is_v_6_1_or_newer)");
    
    if cfg!(feature = "v7_2") {
        println!("cargo:rustc-cfg=is_v_7_2");
        print_7_2_or_newer();
    }else if cfg!(feature = "v7_1") {
        println!("cargo:rustc-cfg=is_v_7_1");
        print_7_1_or_newer();
    } else if cfg!(feature = "v7_0") {
        println!("cargo:rustc-cfg=is_v_7_0");
        print_7_0_or_newer();
    } else if cfg!(feature = "v6_1") {
        println!("cargo:rustc-cfg=is_v_6_1");
        print_6_1_or_newer();
    } else {
        print_7_2_or_newer();
    }
}

fn print_6_1_or_newer() {
    println!("cargo:rustc-cfg=is_v_6_1_or_newer");
}

fn print_7_0_or_newer() {
    println!("cargo:rustc-cfg=is_v_7_0_or_newer");
    print_6_1_or_newer();
}

fn print_7_1_or_newer() {
    println!("cargo:rustc-cfg=is_v_7_1_or_newer");
    print_7_0_or_newer();
}

fn print_7_2_or_newer() {
    println!("cargo:rustc-cfg=is_v_7_2_or_newer");
    print_7_1_or_newer();
}
