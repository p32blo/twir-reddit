#!/bin/bash

export PIPENV_VERBOSITY=-1

pipenv run python url.py $1 > url.txt
pipenv run python markdown.py url.txt > out.md

echo "Generated File 'out.md'"