candidate - current selected item in fzf
commandline - part before the cursor position
token - last token from commandline 

testing completions:
`fish -c "complete -C 'cd' | head -20"`
`fish -c "complete -C 'cd ' --no-files"`
`fish -c "functions -v | grep -i completions"`
`fish -c "echo \$fish_complete_path"`
`fish -c "complete -C 'cd '" | cat -A`
`fish -c "complete -C 'cat '" | cat -A`

print the completion tokens for a given command line 
```fish
fish -c 'complete -C $argv[1]' -- 'git switch ' | string unescape | uniq | awk -F '\t' '{ print $1 }'
```
asks fish to evaluate completions for the commandline `git switch` 

descriptions for the commands - fzf-tab has in-line appending for "--" then the description of git commands
- way to check if the fish completions are for commands, etc.

# Fish Script Side:
- Setup error codes to match behaviour
- If length of selection options is 1, just complete without offering fzf menu
- Decide on the delimiters for string joining
- Running executable: `--preview '/path/to/my-fifc-helper preview {} {q}'`
- You decide based on the "current" token how to handle it, rust will just give command selections
- Make keybind for toggling preview for fifc, default is dropdown
- If you are supporting descriptions, check the description.rs for impl details

# Future Config Options:
- Direct mapping of mime types to different preview commands can be parsed
- Configuration for supporting multiple selection for commands like directory ones as only default (keybind for tab changes from multiselect to acccept)
- FZF Preview options
- Preview commands/options
- Option to override existing command flags with your own

# Default Config Options:
- Flags for each file type preview default command 
```
cd <tab> should be fd (directory only)
ls <tab> should be regular fd
cat/bat should be fd (file only)
export: grab env vars from fish shell
unset: environment variables
unalias: alias command
ssh: only grep entires from cat ~/.ssh/config and tailscale results
kill/pkill: procs 
```

```
cd)           fzf --preview 'tree -C {} | head -200'   "$@" ;;
export|unset) fzf --preview "eval 'echo \$'{}"         "$@" ;;
ssh)          fzf --preview 'dig {}'                   "$@" ;;
*)            fzf --preview 'bat -n --color=always {}' "$@" ;;
```

supporting multiple selections from fzf: 
```
    set -l joined (string join ' ' $choices)
    commandline -i -- $joined
```


# Descriptions:
fzf does not have a separate argument or field for a “description.”
Anything you want to display must be part of the candidate lines you feed into fzf.
So yes — if you want an item to have a label, description, metadata, etc., you must append it to the selection options yourself.

fzf can split an input line into fields (using -d), but it does not store them as separate objects with dedicated meanings. They’re just text columns.

`-d \t` means treat tab as a delimiter

means “treat tab as the delimiter,” so a line like: `filename.txt<TAB>Description of file`

will render two columns, but both are still part of the selectable text. The selected result will be the entire line unless you use --with-nth or --nth to hide or restrict fields, or --preview to do something with the fields.

and then use:
--with-nth to show only some fields,
--preview to interpret fields,
positional placeholders like {1}, {2}, {} etc.

`fzf --with-nth=2,3 --delimiter '\t'`

