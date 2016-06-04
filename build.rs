fn main() {
    println!("cargo:rustc-link-lib=static=compiler-rt");
    println!("cargo:rustc-link-search=native=sysroot/lib/rustlib/3ds.json/lib/");
}
