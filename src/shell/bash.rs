pub const CONFIG: &str = "~/.bashrc";

pub const HOOK: &str = r#"
declare -f -F rsvenv > /dev/null && unset -f rsvenv

RS_VENV_PATH=$(which rsvenv)

_rs_venv_virtualenv_hook () {
    eval "$($RS_VENV_PATH hook)"
}

rsvenv () {
    ALL_PARAMS=($@)
    COMMAND=$1
    SUBPARAMS=("${ALL_PARAMS[@]:1}")
    case $COMMAND in
    "activate")
        eval "$($RS_VENV_PATH activate $SUBPARAMS)"
        ;;
    "deactivate")
        eval "$($RS_VENV_PATH deactivate $SUBPARAMS)"
        ;;
    "delete")
        eval "$($RS_VENV_PATH delete $SUBPARAMS)"
        ;;
    "use")
        eval "$($RS_VENV_PATH use $SUBPARAMS)"
        ;;
    *)
        $RS_VENV_PATH $ALL_PARAMS
        ;;
    esac
}

if ! [[ "\${PROMPT_COMMAND-}" =~ _rs_venv_virtualenv_hook ]]; then
  PROMPT_COMMAND="_rs_venv_virtualenv_hook;\${PROMPT_COMMAND-}"
fi
"#;

pub static ACTIVATE_TEMPLATE: &str = r#"
source {activate_path}
export RSVENV_ACTIVATE_PATH={current_directory}
"#;

pub static DEACTIVATE_TEMPLATE: &str = r#"
unset RSVENV_DEACTIVATE_PATH
deactivate
{{ if forced }}export RSVENV_DEACTIVATE_PATH=$RSVENV_ACTIVATE_PATH{{ endif }}
"#;

pub static INIT_COMMAND: &str = r#"eval "$(rsvenv init)""#;
