printf "\033[34mChecking for TODOs ('TODO' and 'todo!')\033[0m\n"
grep -rn --color=auto "TODO\|todo!" src examples

printf "\n\033[34mChecking for notes ('//?')\033[0m\n"
grep -rn --color=auto "//?" src examples

echo ""
