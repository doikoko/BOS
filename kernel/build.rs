use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;
use std::env::var;

// rustup target add x86_64-unknown-none
fn command<T: AsRef<str>>(com: T, msg: T, error: T){
    println!("{}", msg.as_ref());

    let v1: Vec<&str> = com.as_ref()
        .split_whitespace()
        .collect();
    let mut v2: Vec<String> = Vec::new();

    for i in 0..v1.len(){
        v2.push(String::from(*v1.get(i)
            .unwrap()));
    }

    let status = Command::new(v2.get(0).unwrap())
        .args(v2[1..v2.len()].to_vec())
        .status()
        .unwrap();

    if !status.success(){
        panic!("{}", error.as_ref());
    }
    
} 

fn main(){
    let argv: String;

    if var("CARGO_PKG_NAME").unwrap() != "kernel" {
        panic!("error");
    }
    if var("CARGO_FEATURE_NEW").is_ok(){
        argv = String::from("new");
    } else if var("CARGO_FEATURE_CLEAN").is_ok(){
        argv = String::from("clean");
    } else {
        panic!("error");
    }

    let out_dir = var("OUT_DIR").unwrap();
    let linker_script = PathBuf::from("src").join("kernel.ld");
    
    let loader: String = String::from("loader");
    
    if argv == String::from("new"){
        //loader.ko
        command(format!("nasm -f bin ../loader/{loader}.asm -o ../loader/{loader}.bin"), 
            format!("compiling ../loader/{loader}.asm file, creating bin..."),
            format!("can't compile ../{loader}.asm"));

        command(format!("dd if=../loader/{loader}.bin of=../iso/boot/loader/{loader}.ko \
                        bs=2048 conv=sync"),
            format!("building ../iso/boot/loader/{loader}.ko file..."),
            format!("can't build ../iso/boot/loader/{loader}.ko file from ../loader/{loader}.bin"));
        
        command(format!("rm -f ../loader/{loader}.bin"),
            format!("deleting bin file..."),
            format!("can't delete file ../loader/{loader}.bin"));
        //libraries
        let libraries: [&str; 2] = ["../ports/ports", "../io/io"]; 
        let mut libraries_to_staticlib = String::new();

        for library in &libraries{
            command(format!("nasm -f elf64 {library}.asm -o {library}.o"),
                format!("compiling {library}.asm to object file"),
                format!("can't compile {library}.asm to object file\n
                    maybe you haven't nasm compiler"));

            libraries_to_staticlib = format!("{libraries_to_staticlib} {library}.o ");
        }
        //kernel
        command(format!("ld -r {libraries_to_staticlib} -o {out_dir}/libio.a"),
            format!("creating a static library libio.a to kernel"), 
            format!("can't create a static library libio.a from {libraries_to_staticlib} using ld"));

        command(format!("rustc ./src/main.rs
            -C linker-flavor=ld
            -C link-arg=-T{}
            -C link-arg=-L{out_dir}
            -C link-arg=-lio
            -C link-arg=-e_start
            -o ../iso/boot/kernel.elf", linker_script.display()),
        format!("compilation kernel with libio.a"),
        format!("error while compilate kernel"));
        //iso
        let prog = String::from("xorriso -as mkisof");
        command(format!("{prog} -R -J 
            -b /boot/{loader}.ko -no-emul-boot -boot-load-size 4 
            -o BOS.iso ./iso"), 
            format!("generating iso image using xorriso as mkisofs"),
            format!("generating iso error"));
    }
    else if argv == String::from("clean"){
        remove_file(format!("../iso/boot/loader/{loader}.ko"))
            .expect("no generated files");
        
        remove_file("../iso/boot/kernel.elf")
            .expect("no kernel complete file");

        remove_file("BOS.iso")
            .expect("no iso file");
    }
}