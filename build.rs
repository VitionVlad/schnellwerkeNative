use std::process::Command;
use std::env;

fn main(){
    let out_dir = env::var("OUT_DIR").unwrap();
    
    Command::new("gcc").args(&["src/engine/render/euclidc/euclid.c", "-c", "", "-o"])
                       .arg(&format!("{}/euclid.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["rcs", &format!("{}/libeuclid.a", out_dir), &format!("{}/euclid.o", out_dir)])
                      .status().unwrap();

    Command::new("gcc").args(&["src/engine/audio/mozartc/mozart.c", "-c", "", "-o"])
                       .arg(&format!("{}/mozart.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["rcs", &format!("{}/libmozart.a", out_dir), &format!("{}/mozart.o", out_dir)])
                      .status().unwrap();

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=euclid");
    println!("cargo::rustc-link-lib=static=mozart");
    println!("cargo::rustc-link-lib=vulkan-1");
    println!("cargo::rustc-link-lib=glfw3");
    println!("cargo::rerun-if-changed=src/engine/render/euclidc/euclid.c");
}