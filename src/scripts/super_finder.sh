#!/bin/sh

workpath_file=~/.workpath

directories=$(awk '/\/$/' $workpath_file | while read -r line; do eval echo "$line"; done)
files=$(awk '!/\/$/' $workpath_file | while read -r line; do eval echo "$line"; done)

selected=$((fd -H . $directories ; readlink -e $files) | fzf)

if [[ -z $selected ]]; then
	exit 0
fi

# NOT NECESSARY RIGHT NOW, BUT BETTER KEEP THIS
# if [[ ! -d $selected ]]; then
# 	workdir=$(dirname $selected)
# else 
# 	workdir=$selected
# fi

echo $selected
