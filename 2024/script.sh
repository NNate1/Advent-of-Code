#!/bin/bash
clean() {
	for dir in day-*; do
		if [ -d "$dir" ]; then
			(
				cd "$dir" || exit
				if [ -d "target" ]; then

					echo "Cleaning $dir"
					cargo clean
				fi
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
