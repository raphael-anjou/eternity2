"""Shared loader for solvers.toml — the benchmark's single source of truth.

Both run_grid.py and make_site_json.py import this so there is exactly ONE parser
and one place that knows the manifest schema. See ../solvers.toml for the field
docs. Uses stdlib tomllib (Python 3.11+).
"""
import json
import tomllib
from pathlib import Path

# scripts/ -> experiment root is one up; the manifest is at the root.
_HERE = Path(__file__).resolve().parent
_EXPERIMENT = _HERE.parent
_MANIFEST = _EXPERIMENT / "solvers.toml"
# ...<exp> -> experiments -> research -> <repo> -> web/.../authors.json
_AUTHORS_JSON = _EXPERIMENT.parents[2] / "web" / "content" / "research" / "authors.json"

_REQUIRED = ("name", "author", "family", "kind", "input", "nps_unit", "eligible")
_KINDS = {"native", "standalone"}

# Only Raphaël's own engines get the ownership prefix on the site; every other
# author is credited by name and shown bare (they are not "anjou-" work).
# NOTE: `author` means "who wrote the code we run", NOT "who invented the
# algorithm". Every engine in this grid is currently Raphaël's own code, so all
# of them are prefixed -- including `blackwood`/`verhaard`, which implement the
# community's published algorithms from scratch. Those credit the originator via
# the optional `algorithm_by` field. If a third-party binary is ever run here,
# it gets that author's slug and shows bare.
PREFIX = "anjou-"
_PREFIXED_AUTHOR = "raphael-anjou"


def _known_author_slugs() -> set[str]:
    """Author slugs defined in the site's authors.json (source of truth)."""
    data = json.loads(_AUTHORS_JSON.read_text())
    return {a["slug"] for a in data["authors"]}


def load(path: Path | str | None = None) -> list[dict]:
    """Return the list of solver dicts, validated. Raises on a malformed file."""
    p = Path(path) if path else _MANIFEST
    data = tomllib.loads(p.read_text())
    solvers = data.get("solver", [])
    if not solvers:
        raise ValueError(f"{p}: no [[solver]] entries")
    known_authors = _known_author_slugs()
    seen = set()
    for s in solvers:
        missing = [k for k in _REQUIRED if k not in s]
        if missing:
            raise ValueError(f"{p}: solver {s.get('name', '?')} missing {missing}")
        if s["kind"] not in _KINDS:
            raise ValueError(f"{p}: solver {s['name']} bad kind {s['kind']!r}")
        if s["author"] not in known_authors:
            raise ValueError(
                f"{p}: solver {s['name']} author {s['author']!r} is not a slug in "
                f"authors.json ({sorted(known_authors)})")
        # Optional, and validated the same way: who invented the algorithm, when
        # that is not who wrote the code we run. Never affects site_name.
        if "algorithm_by" in s and s["algorithm_by"] not in known_authors:
            raise ValueError(
                f"{p}: solver {s['name']} algorithm_by {s['algorithm_by']!r} is not "
                f"a slug in authors.json ({sorted(known_authors)})")
        if s["name"] in seen:
            raise ValueError(f"{p}: duplicate solver name {s['name']!r}")
        seen.add(s["name"])
    return solvers


def site_name(solver: dict) -> str:
    """The name shown on the site: anjou- prefix for Raphaël's, bare otherwise."""
    return PREFIX + solver["name"] if solver["author"] == _PREFIXED_AUTHOR else solver["name"]


def by_kind(solvers: list[dict], kind: str) -> list[str]:
    return [s["name"] for s in solvers if s["kind"] == kind]
