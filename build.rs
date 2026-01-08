use std::process::Command;
use std::env;

//use winresource::WindowsResource;

fn main(){
    //if env::var_os("CARGO_CFG_WINDOWS").is_some() {
    //    let _ = WindowsResource::new()
    //        .set_icon("assets/icon.ico")
    //        .compile();
    //}

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

    Command::new("gcc").args(&["src/engine/loader/vspng/spng.c", "-c", "", "-o"])
                       .arg(&format!("{}/spng.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["rcs", &format!("{}/libspng.a", out_dir), &format!("{}/spng.o", out_dir)])
                      .status().unwrap();

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=euclid");
    println!("cargo::rustc-link-lib=static=mozart");
    println!("cargo::rustc-link-lib=static=spng");
    println!("cargo::rustc-link-lib=vulkan-1");
    println!("cargo::rustc-link-lib=glfw3");
    println!("cargo::rustc-link-lib=png");
    println!("cargo::rerun-if-changed=src/engine/render/euclidc/euclid.c");
    println!("cargo::rerun-if-changed=src/engine/audio/mozartc/mozart.c");
    println!("cargo::rerun-if-changed=src/engine/loader/vspng/spng.c");
}