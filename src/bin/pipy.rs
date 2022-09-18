fn main() {
    let mut args = std::env::args();
    args.next(); // drop the `pipy` arg
    let args = args.collect::<Vec<String>>();
    let mut pip_args = vec!["-m".to_owned(), "pip".to_owned()];
    pip_args.extend(args);
    run(pip_args);
}

#[cfg(target_os = "windows")]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python.exe", args);
}

#[cfg(not(target_os = "windows"))]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python3", args);
}
