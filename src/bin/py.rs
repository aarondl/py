fn main() {
    let mut args = std::env::args();
    args.next();

    pylib::run_command_with_args("python3", args.collect()).unwrap();
}
