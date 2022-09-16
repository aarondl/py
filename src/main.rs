fn main() {
    let mut current = std::env::current_dir().expect("can get working dir");
    loop {
        let mut py_mod_path = current.clone();
        py_mod_path.push("py.mod");
        if let Ok(_) = std::fs::metadata(py_mod_path) {
            break;
        } else {
            if !current.pop() {
                println!("failed to find py.mod file");
                std::process::exit(1);
            }
        }
    }

    let mut args = std::env::args();
    args.next();

    let mut cmd = std::process::Command::new("python");
    cmd.env("PYTHONPATH", current);
    cmd.args(args);
    let mut handle = cmd.spawn().expect("should be able to run python");
    handle.wait().expect("failed running process");
}
