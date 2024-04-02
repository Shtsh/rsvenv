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

typeset -g -a precmd_functions

if [[ -z $precmd_functions[(r)_rs_venv_virtualenv_hook] ]]; then
  precmd_functions=(_rs_venv_virtualenv_hook $precmd_functions);
fi
"#;
