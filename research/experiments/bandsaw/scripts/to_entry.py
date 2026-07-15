#!/usr/bin/env python3
"""Adapt dump_board_json output into the placement JSON mini_e2 --entry expects.

dump_board_json emits {"board": [{pos, piece_id, rotation}, ...], ...}; mini_e2
reads {"placement": [...]}. This just rewraps the array under the expected key,
so BANDSAW can take any board the benchmark engine can decode.

    dump_board_json <url> > raw.json
    python3 to_entry.py raw.json > entry.json
"""
import json
import sys

raw = json.load(open(sys.argv[1]))
placement = raw.get("placement") or raw["board"]
json.dump({"placement": placement}, sys.stdout)
