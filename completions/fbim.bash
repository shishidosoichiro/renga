# bash completion for fbim
# Source this file or place it in /etc/bash_completion.d/

# Walk up from PWD to find issues directory (mirrors bin/fbim logic).
_fbim_find_issues_dir() {
    local dir="$PWD"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/.fbim.yml" ]]; then
            local rel
            rel=$(grep -m1 '^issues_dir' "$dir/.fbim.yml" 2>/dev/null \
                | sed 's/^issues_dir[[:space:]]*:[[:space:]]*//' \
                | tr -d '"'"'")
            echo "${dir}/${rel:-issues}"
            return 0
        fi
        if [[ -d "$dir/issues" ]]; then
            echo "$dir/issues"
            return 0
        fi
        dir="${dir%/*}"
    done
    return 1
}

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
                    local issues_dir
                    issues_dir=$(_fbim_find_issues_dir 2>/dev/null)
                    if [[ -n "$issues_dir" && -d "$issues_dir" ]]; then
                        local -a candidates=()
                        while IFS= read -r f; do
                            local stem num title
                            stem=$(basename "$f" .md)
                            num="${stem%%-*}"
                            title=$(grep -m1 '^# ' "$f" 2>/dev/null | sed 's/^# //')
                            candidates+=("$num  # ${title:-$stem}")
                        done < <(ls "$issues_dir"/[0-9][0-9][0-9][0-9]-*.md \
                                    "$issues_dir"/[0-9][0-9][0-9][0-9][0-9]-*.md 2>/dev/null)
                        local nums=()
                        for c in "${candidates[@]}"; do
                            nums+=("${c%%  *}")
                        done
                        COMPREPLY=($(compgen -W "${nums[*]}" -- "$cur"))
                    fi
                    ;;
                reopen)
                    local issues_dir
                    issues_dir=$(_fbim_find_issues_dir 2>/dev/null)
                    if [[ -n "$issues_dir" && -d "$issues_dir/done" ]]; then
                        local nums
                        nums=$(ls "$issues_dir/done"/[0-9][0-9][0-9][0-9]-*.md \
                                  "$issues_dir/done"/[0-9][0-9][0-9][0-9][0-9]-*.md 2>/dev/null \
                            | xargs -I{} basename {} .md \
                            | sed 's/-.*//')
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
