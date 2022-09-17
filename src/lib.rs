use std::io;
use std::path::{Path, PathBuf};

pub fn find_python_mod_dir() -> io::Result<Option<(PathBuf, Vec<u8>)>> {
    let mut current = std::env::current_dir().expect("can get working dir");
    loop {
        let mut py_mod_path = current.clone();
        py_mod_path.push("py.mod");
        if let Ok(bytes) = std::fs::read(py_mod_path) {
            return Ok(Some((current, bytes)));
        } else {
            if !current.pop() {
                return Ok(None);
            }
        }
    }
}

fn venv_exists<P: AsRef<Path>, D: AsRef<Path>>(root: P, venv_dir: D) -> Option<PathBuf> {
    let mut envpath = PathBuf::from(root.as_ref());
    envpath.push(venv_dir.as_ref());
    let mut envcfgpath = envpath.clone();
    envcfgpath.push("pyvenv.cfg");
    match std::fs::metadata(&envcfgpath) {
        Ok(_) => Some(envpath),
        Err(_) => None,
    }
}

pub fn run_command_with_args<P: AsRef<Path>>(cmd: P, args: Vec<String>) {
    // If we don't have a py.mod then just run the command as normal
    let (python_mod_dir, venv_dir) = match find_python_mod_dir().expect("should be able to check for mod dir") {
        Some((path, bytes)) => {
            let venv_dir = String::from_utf8(bytes).expect("the contents of py.mod to be valid utf8");
            let venv_dir = venv_dir.trim().to_owned();
            if venv_dir.is_empty() {
                (path, "env".to_owned())
            } else {
                (path, venv_dir)
            }
        }
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
    let venv_dir = venv_exists(&python_mod_dir, &venv_dir);

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
        cmd.env("VIRTUAL_ENV", &venv_dir);
        eprintln!("venv: {}", venv_dir.to_string_lossy());

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

#[cfg(windows)]
fn append_venv_bin_path(p: &mut PathBuf) {
    p.push("Scripts");
}

#[cfg(not(target_os = "windows"))]
fn append_venv_bin_path(p: &mut PathBuf) {
    p.push("bin");
}
