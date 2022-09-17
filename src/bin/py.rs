fn main() {
    let mut args = std::env::args();
    args.next();

    run(args.collect());
}

#[cfg(target_os = "windows")]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python.exe", args);
}

#[cfg(not(target_os = "windows"))]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python3", args);
}
