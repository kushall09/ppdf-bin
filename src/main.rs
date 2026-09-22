use std::env;
use std::process::Command;
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    println!("Converting {input} into pdf...");
    let process = Command::new("soffice")
           .args(&["--headless", "--convert-to", "pdf", "--outdir", "done", input])
           .status()
           .expect("failed to execute command");
    println!("{}", process);
}
