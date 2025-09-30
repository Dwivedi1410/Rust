fn main() {
    println!("Hello, world!");
    println!("Changed the main.rs file")
}

// Note:- cargo dosn't watches your file changes instead you have to use plugins like cargo-watch for this operation.

// Note:- you can use "rustup update" to update the rust version.

// Note:- To see the preinstalled documents use "rustup doc"

// Note:- To compile any file you have to use "rustc main.rc" and to run that file just enter the location of the file(executable file)

// Note:- To build a project by using cargo use this cammand "cargo new project_name"
// and to build this project use this command "cargo build" this will create a executable file in ./target/debug/project_name and to run this file give the location in the terminal

// Note:- To compile and run the cargo file we use the command "crago run"

// Note:- We can use "cargo check" to check this code compiles but dosn't create an executable.

// Note:- when we have compleated the project at final we do "cargo build --release" to compile it with optimization , this command will create executable in target/release instead of target/debug, this executable make sure that the rust code runs faster but it takes time to compile this code.