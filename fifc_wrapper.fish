function fifc_wrapper --description "Context-aware fifc wrapper"
    set -l tokens (commandline -xpc)
    set -l current (commandline -ct)

    # echo tokens: $tokens
    # echo current: $current

    if test (count $tokens) -eq 0
        _fifc
        return
    end

    # Detect `git switch -C …` specifically
    if test "$tokens[1]" = git -a (count $tokens) -ge 2 -a "$tokens[2]" = switch
        # Guard for inside a git repo
        if not command git rev-parse --is-inside-work-tree >/dev/null 2>/dev/null
            return
        end

        # -C flag is already in the tokens or you are currently typing it out
        if contains -- -C $tokens || string match -q -- '-C*' -- $current
            __my_git_switch_C_fzf $current
        else
            __my_git_switch_fzf $current
        end
        return
    end

    _fifc
end

function __my_git_switch_fzf
    set -l current $argv[1]
    set -l local_branches (command git branch --format="%(refname:short)")

    set -l selection (printf '%s\n' $local_branches | fzf --query "$current")
    commandline --function repaint
    test -z "$selection"; and return

    commandline --insert -- "$selection"
    commandline --function repaint
end

function __my_git_switch_C_fzf
    set -l current $argv[1]

    set -l local_branches (command git branch --format='%(refname:short)')
    set -l choices
    for ref in (command git for-each-ref --format='%(refname:short)' refs/remotes/origin)
        set -l name (string replace 'origin/' '' -- $ref)
        if not contains -- $name $local_branches
            set -a choices $name # Append to the list
        end
    end

    set -l selection (printf '%s\n' $choices | fzf --query "$current")
    commandline --function repaint
    test -z "$selection"; and return

    commandline --insert -- "$selection"
    commandline --function repaint
end
