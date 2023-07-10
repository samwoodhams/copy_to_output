# copy_to_output
A small rust library to copy files/folders from the project directory to the output directory  

## Copy file/folder
`copy_to_output("path name", "build profile");`

## When building
You can use a build.rs file to copy a file/folder to the output directory when running cargo build  

**Example build.rs**  
```rs
 use std::env;  
 use copy_to_output::copy_to_output;  
   
 fn main() {  
     // Re-runs script if any files in res are changed  
     println!("cargo:rerun-if-changed=res/*");  
     copy_to_output("res", &env::var("PROFILE").unwrap()).expect("Could not copy");  
 }
 ``` 
 where 'res' is the name of the directory to copy to the output directory  
  
**Example Cargo.toml**  
```rs
[package]
name = "test_crate"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[build-dependencies]
copy_to_output = "2.1.0"
glob = "0.3"
 ```
 glob provides the * wildcard operator for the println!() line in build.rs  
   
 Both of these examples are in the example_files folder in this repo  
