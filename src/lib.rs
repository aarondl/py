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

fn venv_exists<P: AsRef<Path>>(root: P) -> bool {
    let mut envpath = PathBuf::from(root.as_ref());
    envpath.push("env/pyenv.cfg");
    return std::fs::metadata(root).is_ok();
}

pub fn run_command_with_args<P: AsRef<Path>>(cmd: P, args: Vec<String>) -> io::Result<()> {
    // If there's no Python mod dir, just run Python as normal
    let python_mod_dir = match find_python_mod_dir()? {
        Some(d) => d,
        None => {
            eprintln!("mod: none");
            let mut cmd = std::process::Command::new(cmd.as_ref());
            cmd.args(args);
            let mut handle = cmd.spawn()?;
            handle.wait()?;
            return Ok(());
        }
    };
    eprintln!("mod: {:?}", python_mod_dir);

    let is_venv = venv_exists(&python_mod_dir);

    let command_path = if is_venv {
        // Look for all commands inside venv bin
        let mut cmd_path = python_mod_dir.clone();
        cmd_path.push("env/bin");
        cmd_path.push(cmd);
        cmd_path
    } else {
        PathBuf::from(cmd.as_ref()) // just rely on $PATH
    };

    let mut cmd = std::process::Command::new(command_path);

    cmd.env("PYTHONPATH", &python_mod_dir);
    if is_venv {
        cmd.env("VIRTUAL_ENV", &python_mod_dir);
        eprintln!("found venv");

        // set this in case other things use $PATH during execution
        let mut venv_bin_path = python_mod_dir.clone();
        venv_bin_path.push("env/bin");

        let current_path = std::env::var("PATH").expect("should be able to get PATH var");
        let current_paths = std::env::split_paths(&current_path);
        let path_env = std::env::join_paths([venv_bin_path].into_iter().chain(current_paths)).unwrap();
        cmd.env("PATH", path_env);
    }

    cmd.args(args);
    let mut handle = cmd.spawn()?;
    handle.wait()?;

    Ok(())
}
