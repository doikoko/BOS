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
    
    if var("CARGO_FEATURE_NEW").is_ok(){
        argv = String::from("new");
    } else if var("CARGO_FEATURE_CLEAN").is_ok(){
        argv = String::from("clean");
    } else {
        panic!("error");
    }

    let out_dir = var("OUT_DIR").unwrap();

    if argv == String::from("new"){
        //loader.ko
        let file: String = String::from("loader");
        command(format!("nasm -f bin ./loader/{file}.asm -o ./loader/{file}.bin"), 
            format!("compiling ./loader/{file}.asm file, creating bin..."),
            format!("can't compile ./{file}.asm"));

        command(format!("dd if=./loader/{file}.bin of=./iso/boot/loader/{file}.ko \
                        bs=2048 conv=sync"),
            format!("building ./iso/boot/loader/{file}.ko file..."),
            format!("can't build ./iso/boot/loader/{file}.ko file from ./loader/{file}.bin"));
        
        command(format!("rm -f ./loader/{file}.bin"),
            format!("deleting bin file..."),
            format!("can't delete file ./loader/{file}.bin"));
        //libraries
        let libraries: [&str; 2] = ["./ports/ports", "./io/io"]; 
        let mut libraries_to_staticlib: String = String::new();

        for library in &libraries{
            command(format!("nasm -f elf64 {library}.asm -o {library}.o"),
                format!("compiling {library}.asm to object file"),
                format!("can't compile {library}.asm to object file"));

            libraries_to_staticlib = format!("{libraries_to_staticlib}{library}.io ");
        }
        //kernel
        command(format!("ld -r {libraries_to_staticlib} -o {out_dir}/libio.a"),
            format!("creating a static library libio.a to kernel"), 
            format!("can't create a static library libio.a from {libraries_to_staticlib}"));
        
        println!("cargo:rustc-link-search=native={out_dir}");
        println!("cargo:rustc-link-lib=static=io");
    }
}