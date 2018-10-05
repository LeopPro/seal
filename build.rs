use std::process::Command;

fn execute_front_end_build_sh(){
    println!("`execute sh src/front_end/build.sh`");
    let output = match Command::new("sh").arg("src/front_end/build.sh").output() {
        Ok(output) => output,
        Err(err) => panic!("Panic at `sh src/front_end/build.sh`\n{}", err)
    };
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    } else {
        panic!("`sh src/front_end/build.sh` return failure\n{}", String::from_utf8(output.stderr).unwrap());
    }
}

fn main() {
    execute_front_end_build_sh();
}