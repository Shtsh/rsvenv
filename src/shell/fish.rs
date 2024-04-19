pub const CONFIG: &str = "~/.config/fish/config.fish";

pub const HOOK: &str = r#"
if set -q rsvenv;
    set -e rsvenv
end

set -g RS_VENV_PATH (which rsvenv)

function _rs_venv_virtualenv_hook --on-event fish_prompt;
   $RS_VENV_PATH hook | source
end

function rsvenv
    set eval_commands activate deactivate delete use
    if contains $argv[1] $eval_commands
        $RS_VENV_PATH $argv | source
    else
        $RS_VENV_PATH $argv
    end
end
"#;

pub static ACTIVATE_TEMPLATE: &str = r#"
source {activate_path}
set -gx RSVENV_ACTIVATE_PATH {current_directory}
"#;

pub static DEACTIVATE_TEMPLATE: &str = r#"
set -e RSVENV_DEACTIVATE_PATH
deactivate
{{ if forced }}set -gx RSVENV_DEACTIVATE_PATH $RSVENV_ACTIVATE_PATH{{ endif }}
"#;

pub static INIT_COMMAND: &str = "status --is-interactive; and source (rsvenv init |psub)";
