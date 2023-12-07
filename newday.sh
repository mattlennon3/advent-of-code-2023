#!/bin/sh

# e.g previous folder would be named day05a
# next folder would be named day06a

cd "$HOME/git/rust/advent-of-code-2023"

# get the last folder name
lastfolder=$(ls -d day* | tail -1)

# get the last folder number

lastfoldernum=$(echo $lastfolder | sed 's/[^0-9]*//g')

# increment the number

nextfoldernum=$(expr $lastfoldernum + 1)
# create the new folder name "day06a", make sure to add the 0 if it's a single digit

if [ $nextfoldernum -lt 10 ]
then
    nextfoldernum="0$nextfoldernum"
fi

newfoldername=$(echo $lastfolder | sed "s/$lastfoldernum/$nextfoldernum/g")

# create the new folder

mkdir $newfoldername

# cd to folder and run "cargo init"

cd $newfoldername

touch input.txt

cargo init

cargo add --path ../aoc-crate

code-insiders src/main.rs

code-insiders input.txt
