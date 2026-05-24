# bash completion for fbim
# Source this file or place it in /etc/bash_completion.d/

_fbim_completion() {
    local cur prev words cword
    _init_completion 2>/dev/null || {
        COMPREPLY=()
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
    }

    local commands="create done pending reopen list show help"

    case $COMP_CWORD in
        1)
            COMPREPLY=($(compgen -W "$commands" -- "$cur"))
            ;;
        *)
            case ${COMP_WORDS[1]} in
                create)
                    case $prev in
                        --priority)
                            COMPREPLY=($(compgen -W "high medium low" -- "$cur"))
                            ;;
                        *)
                            COMPREPLY=($(compgen -W "--slug --priority --area --body" -- "$cur"))
                            ;;
                    esac
                    ;;
                done|pending|show)
                    if [[ -d "$PWD/issues" ]]; then
                        local nums
                        nums=$(ls "$PWD/issues"/[0-9][0-9][0-9][0-9]-*.md \
                                  "$PWD/issues"/[0-9][0-9][0-9][0-9][0-9]-*.md 2>/dev/null \
                            | xargs -I{} basename {} .md \
                            | grep -oE '^[0-9]+')
                        COMPREPLY=($(compgen -W "$nums" -- "$cur"))
                    fi
                    ;;
                reopen)
                    if [[ -d "$PWD/issues/done" ]]; then
                        local nums
                        nums=$(ls "$PWD/issues/done"/[0-9][0-9][0-9][0-9]-*.md \
                                  "$PWD/issues/done"/[0-9][0-9][0-9][0-9][0-9]-*.md 2>/dev/null \
                            | xargs -I{} basename {} .md \
                            | grep -oE '^[0-9]+')
                        COMPREPLY=($(compgen -W "$nums" -- "$cur"))
                    fi
                    ;;
                list)
                    case $prev in
                        --status)
                            COMPREPLY=($(compgen -W "open pending done" -- "$cur"))
                            ;;
                        *)
                            COMPREPLY=($(compgen -W "--json --status --area --label" -- "$cur"))
                            ;;
                    esac
                    ;;
                help)
                    COMPREPLY=($(compgen -W "$commands" -- "$cur"))
                    ;;
            esac
            ;;
    esac
}

complete -F _fbim_completion fbim
