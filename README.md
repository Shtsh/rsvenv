# RSVENV

## Description

Tool to manage python virtual environments. It stores all of them in one place 
and allows to use any python binary to create a new one.

## Features
* Automatic venv activation when changing the directory (in bash and zsh)
* Support of virtual environments created via [pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv/tree/master)
* Supports virtual environments in current directory (subdirectory must be named one of .venv, .virtualenv, venv, virtualenv)

## Usage

In order to use automatic virtual environment activation it is necessary to add to the end of ~/.zshrc (or ~/.bashrc)
```bash
eval "$(rsvenv init)"
```

To see available virtual environments 
```bash
rsvenv list
```
Result will be something like this
```
Rsenv environments:
	3.11.8/test_rsenv
Pyenv environments:
	3.12.1/envs/armis
	3.9.9/envs/proxy_pac
	3.12.1/envs/kafka
	3.9.9/envs/jira
```

To create a new virtual environment
```bash
rsvenv create path/to/python venv_name
```
The created virtual environment name will be python_version/venv_name


After this it is possible to use this venv in current directory
```bash
rsvenv use venv_name
```
This command will create file .python-virtualenv that contains venv name

It is possible to deactivate ven
```bash
rsvenv deactivate
```

## Configuration

It is possible to configure parameters via environment variables

| Variable         | Possible values | Default value | Description                                                                       |
|------------------|----------------|--------------|-----------------------------------------------------------------------------------|
| $RSVENV_VERBOSITY | i32 0..3       | 1            | How verbose the program must be: 0 - no messages, 1 - Info level, 2 - Debug level |
| $RSVENV_PATH     | String         | "~/.rsenv"   | Directory to store all the virtual environments|
