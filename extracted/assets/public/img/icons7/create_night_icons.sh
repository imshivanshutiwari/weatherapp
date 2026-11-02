#!/bin/bash

# Run this script to generate all night variants from a daytime icon
#
# If only XXX.png exists, create all XXX_night_P.png moon phase variants

n_weather_variants=27
n_moon_phases=8

directories=$(find . ! -path . -type d)

for dir in $directories; do
  for variant in $(seq $n_weather_variants); do
    if [[ -n $(find $dir -name "${variant}_night*") ]]; then
      # Night versions already exist, skip
      continue
    fi

    echo "Creating variant ${variant} in directory ${dir}"

    filename=$(find $dir -name "${variant}.*")
    extension=${filename##*.}

    for p in $(seq $n_moon_phases); do
      ln -s "${variant}.${extension}" "${dir}/${variant}_night_${p}.${extension}"
    done
  done
done
