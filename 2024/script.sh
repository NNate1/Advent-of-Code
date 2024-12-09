#!/bin/bash
clean() {
	for dir in day-*; do
		echo "$dir"
		if [ -d "$dir" ]; then
			(
				cd "$dir" || exit
				cargo clean
			)
		fi
	done
}

while [[ $# -gt 0 ]]; do
	case $1 in
	clean)
		clean
		shift
		;;
	esac
done
