#!/bin/bash

# This clears all generated content, skipping the public/files/ directory

clear_dir () {
	if test -d $2
	then
		find $2 -maxdepth $1 -type f -delete
	fi
}

if test -d "public"
then

	clear_dir 1 "public/"
	clear_dir 1 "public/all/"
	clear_dir 1 "public/error/"
	clear_dir 1 "public/static/"

	clear_dir 2 "public/creator/"
	clear_dir 2 "public/thumbnails/"

	find "public/" -maxdepth 2 -type d -empty -delete

fi
