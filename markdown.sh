#!/bin/bash

set -e 
#export PIPENV_VERBOSITY=-1

for file in output/*
do
   if [ -f "$file" ]
   then
        poetry run python src/extract.py "$file" > extract.txt
        poetry run python src/markdown.py extract.txt > "$file.md"

        echo "Generated File '$file.md'"
   fi
done