#!/bin/sh
# usage: ./run.sh <args to dm.py>
exec uv run --quiet --with numpy --with scipy python -I "$(dirname "$0")/dm.py" "$@"
