# py

Proper module management for Python!

Most modern languages have something in the root of a project to define a
project and when run build commands use that context. As an example `cargo test`
knows what you mean even if you're several subdirectories deep and it's easy
to import subdirectories.

Python on the other hand is just a directory tree full of scripts and it doesn't
understand having a project directory root where you likely want to share
imports from that root as well as a virtualenv for all scripts in that directory
tree. This seems downright unpleasant compared to modern tooling in languages
like Go, Rust, Node, Haskell, and Java.

`py` and `pypi` fix this by recursing up the directory tree to find a `py.mod`
and optionally a `py.venv` file and setting your environment for the duration
of the command. This has a better developer experience than activate/deactivate
for venvs or having to set PYTHONPATH constantly.

When you want to run `python(3)` use `py` instead!

When you want to run `pip(3)` use `pipy` instead!

## Using py.mod

For the following examples we're assuming this folder structure:
```bash
# project root lives beside the py.mod
/code/project/py.mod
# virtual env lives here
/code/project/env/pyvenv.cfg
```

In order to use `py.mod` you just need to create an empty `py.mod` file in the
root of your python project.

When you run `py` or `pipy` while you have a `py.mod` file the following effects
occur to fix importing:

```bash
* PYTHONPATH=/code/project
```

If using virtualenvs it is convention when using `py` to create one at the same
directory level as `py.mod` and it should be named `env`. You can do this with
the following command:

```bash
cd my/project
touch py.mod
py -m venv env # you can also use python/python3 instead of py, but bother?!
```

_Note: For customizing the virtualenv location see [py.venv](#py-venv)_

When virtualenvs are in use the following effects occur:

```bash
* PYTHONPATH=/code/project      # same as without a venv
* PYTHONHOME=                   # this gets unset
* VIRTUAL_ENV=/code/project/env
* PATH=$VIRTUAL_ENV/bin:$PATH   # python's venv bin folder is prepended to path
* python/python3 & pip/pip3 run from $VIRTUAL_ENV/bin
```

## Using py.venv

This is a configuration file to set a custom location for a venv. The contents
are a single path to the virtual env folder, it should be relative but can
be absolute.

If creating a venv via the std library and the `py` command, `py` will note
if the venv is not being created in the conventional location (`env`) and
automatically write a `py.venv` for you. See below for an example:

```bash
py -m venv env # does not write py.venv
py -m venv something # writes py.venv with something in it
py -m venv /happy/something # writes py.venv with /happy/something in it
```
