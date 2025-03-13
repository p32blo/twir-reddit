#!/bin/bash

#set -e 
#export PIPENV_VERBOSITY=-1

poetry run python src/url.py $1 > url.txt
poetry run python src/markdown.py url.txt > out.md

echo "Generated File 'out.md'"
