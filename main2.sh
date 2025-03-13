#!/bin/bash

set -e -x
#export PIPENV_VERBOSITY=-1

python src/url.py $1 > url.txt
python src/markdown.py url.txt > out.md

echo "Generated File 'out.md'"
