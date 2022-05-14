fn main() {
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=mlx");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=bsd");
}
