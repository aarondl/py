fn main() {
    let mut args = std::env::args();
    args.next();

    pylib::run_command_with_args("pip3", args.collect()).unwrap();
}
