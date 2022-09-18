fn main() {
    let mut args = std::env::args();
    args.next();

    let args = args.collect::<Vec<String>>();

    // Check to see if we're creating a virtualenv
    let match_args = args.iter().map(String::as_str).collect::<Vec<&str>>();
    match match_args.as_slice() {
        &["-m", "venv", ..] => {
            let path = match_args.into_iter().skip(2).collect::<Vec<&str>>().join(" ");
            if path != "env" {
                pylib::create_py_venv(&path);
            }
        }
        _ => (),
    }

    run(args);
}

#[cfg(target_os = "windows")]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python.exe", args);
}

#[cfg(not(target_os = "windows"))]
fn run(args: Vec<String>) {
    pylib::run_command_with_args("python3", args);
}
