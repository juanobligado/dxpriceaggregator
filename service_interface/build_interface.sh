#!/usr/bin/env bash
rm -f air-scripts/*.*
aqua-cli -i aqua-scripts -o air-scripts -m . -a
aqua-cli -i aqua-scripts -o air-scripts -m .