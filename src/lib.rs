use std::io;
use std::path::{Path, PathBuf};

pub fn find_python_mod_dir() -> io::Result<Option<PathBuf>> {
    let mut current = std::env::current_dir().expect("can get working dir");
    loop {
        let mut py_mod_path = current.clone();
        py_mod_path.push("py.mod");
        if let Ok(_) = std::fs::metadata(py_mod_path) {
            return Ok(Some(current));
        } else {
            if !current.pop() {
                return Ok(None);
            }
        }
    }
}

fn find_venv_dir<P: AsRef<Path>>(py_mod_dir: P) -> Option<PathBuf> {
    // Try to find an env/pyvenv.cfg next to our mod
    // file, this is convention and the best way to use py.
    let mut envpath = PathBuf::from(py_mod_dir.as_ref());
    envpath.push("env");
    let mut pyvenv_cfg_path = envpath.clone();
    pyvenv_cfg_path.push("pyvenv.cfg");
    if std::fs::metadata(&pyvenv_cfg_path).is_ok() {
        return Some(PathBuf::from(envpath));
    }

    // Fallback to configuration. py.venv allows someone to configure
    // where the virtualenv is stored, it just contains a path
    // and should be gitignored.
    let mut envpath = PathBuf::from(py_mod_dir.as_ref());
    envpath.push("py.venv");
    match std::fs::read(&envpath) {
        Ok(contents) => {
            let venv_dir = std::str::from_utf8(&contents).expect("py.venv contents to be utf8");
            let path = PathBuf::from(venv_dir.trim());
            path.canonicalize().ok()
        }
        _ => None,
    }
}

pub fn run_command_with_args<P: AsRef<Path>>(cmd: P, args: Vec<String>) {
    // If we don't have a py.mod then just run the command as normal
    let python_mod_dir = match find_python_mod_dir().expect("should be able to check for mod dir") {
        Some(path) => path,
        None => {
            eprintln!("mod: none");
            let mut cmd = std::process::Command::new(cmd.as_ref());
            cmd.args(args);
            let mut handle = cmd.spawn().expect("can run the command specified");
            handle.wait().expect("can wait on the command");
            return;
        }
    };

    eprintln!("mod: {}", python_mod_dir.to_str().expect("to be able to show mod path as utf8"));

    // Find a venv if there is one configured
    let venv_dir = find_venv_dir(&python_mod_dir);
    let command_path = match venv_dir {
        Some(ref venv_dir) => {
            // Look for all commands inside venv bin
            let mut cmd_path = venv_dir.clone();
            append_venv_bin_path(&mut cmd_path);
            cmd_path.push(cmd.as_ref());
            cmd_path
        }
        None => PathBuf::from(cmd.as_ref()), // just rely on $PATH
    };

    let mut cmd = std::process::Command::new(&command_path);

    cmd.env("PYTHONPATH", &python_mod_dir);
    if let Some(venv_dir) = venv_dir {
        eprintln!("venv: {}", venv_dir.to_string_lossy());

        cmd.env("VIRTUAL_ENV", &venv_dir);
        cmd.env_remove("PYTHONHOME");

        // set this in case other things use $PATH during execution
        let mut venv_bin_path = venv_dir.clone();
        append_venv_bin_path(&mut venv_bin_path);

        let current_path = std::env::var("PATH").expect("should be able to get PATH var");
        let current_paths = std::env::split_paths(&current_path);
        let path_env = std::env::join_paths([venv_bin_path].into_iter().chain(current_paths))
            .expect("should be able to join paths");
        cmd.env("PATH", path_env);
    }

    cmd.args(args);
    let mut handle =
        cmd.spawn().expect(&format!("should be able to spawn command: {}", command_path.to_str().unwrap(),));
    handle.wait().expect("should be able to wait on command");
}

// create_py_mod is invoked when trying to create a virtual env
// with a specific path
pub fn create_py_venv(name: &str) {
    std::fs::write("py.venv", name.as_bytes()).expect("to be able to write py.venv");
}

#[cfg(windows)]
fn append_venv_bin_path(p: &mut PathBuf) {
    p.push("Scripts");
}

#[cfg(not(target_os = "windows"))]
fn append_venv_bin_path(p: &mut PathBuf) {
    p.push("bin");
}
