#!/usr/bin/env python3
"""Extend the local groups.io archive to the current tail via the REST API.

The bulk archive (../../research/community-exports/messages.jsonl, one message
per line) lives OUTSIDE this repo and is never committed (large + third-party).
This script tops it up: it enumerates the eternity2 group's topics newest-first,
pulls each topic's full messages via `gettopic`, and appends every message whose
`msg_num` is greater than the archive's current maximum. Idempotent — messages
already present are skipped, so re-running only adds what is new.

Credentials: GROUPS_IO_SCRAPING_KEY from the parent .env (see GROUPSIO_API.md).
Usage:
    set -a; . ../.env; set +a          # from the repo root
    python3 research/community/refresh_archive.py

Deleted/moderated posts leave permanent gaps in the msg_num sequence (they are
not served by the API); the script reports the gaps it observes so a reviewer
can tell a real absence from a fetch miss.
"""
import json
import os
import sys
import time
import urllib.parse
import urllib.request

KEY = os.environ.get("GROUPS_IO_SCRAPING_KEY")
if not KEY:
    sys.exit("GROUPS_IO_SCRAPING_KEY not set (source the parent .env first)")

GROUP = 41375
BASE = "https://groups.io/api/v1"
HERE = os.path.dirname(os.path.abspath(__file__))
ARCHIVE = os.path.normpath(
    os.path.join(HERE, "..", "..", "..", "research", "community-exports", "messages.jsonl")
)
# Stop enumerating once this many consecutive topics are entirely older than the
# frontier and contribute no new message. Topics are recency-sorted, so a long
# run of old, contributionless topics means we have passed the frontier.
STOP_STREAK = 40


def api(path, params):
    url = f"{BASE}/{path}?" + urllib.parse.urlencode(params)
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {KEY}"})
    for attempt in range(5):
        try:
            with urllib.request.urlopen(req, timeout=30) as r:
                return json.load(r)
        except Exception:
            if attempt == 4:
                raise
            time.sleep(2 * (attempt + 1))


def topics_newest_first():
    params = {
        "group_id": GROUP,
        "limit": 40,
        "sort_field": "lastpost_time",
        "sort_dir": "desc",
    }
    while True:
        d = api("gettopics", params)
        for t in d.get("data", []):
            yield t
        if not d.get("has_more"):
            break
        params["page_token"] = d["next_page_token"]


def topic_messages(topic_id):
    params = {"topic_id": topic_id, "limit": 50}
    while True:
        d = api("gettopic", params)
        for m in d.get("data", []):
            yield m
        if not d.get("has_more"):
            break
        params["page_token"] = d["next_page_token"]


def main():
    if not os.path.exists(ARCHIVE):
        sys.exit(f"archive not found at {ARCHIVE}")
    existing = set()
    cutoff = 0
    for line in open(ARCHIVE):
        line = line.strip()
        if not line:
            continue
        try:
            n = json.loads(line).get("msg_num")
        except Exception:
            continue
        if n:
            existing.add(n)
            cutoff = max(cutoff, n)
    cutoff_date = None  # derived below from the frontier topic dates
    sys.stderr.write(f"archive: {len(existing)} messages, max msg_num {cutoff}\n")

    new_msgs = {}
    seen = 0
    streak = 0
    for t in topics_newest_first():
        seen += 1
        recent = (t.get("most_recent_message") or "")[:10]
        got_new = False
        try:
            for m in topic_messages(t["id"]):
                n = m.get("msg_num")
                if n and n > cutoff and n not in existing:
                    new_msgs[n] = m
                    got_new = True
        except Exception as e:
            sys.stderr.write(f"  topic {t['id']} error: {e}\n")
        # a topic whose newest post predates the current tail and adds nothing
        # counts toward the stop streak
        if not got_new and recent and (not new_msgs or recent < "2000-01-02"):
            streak += 1
        elif not got_new:
            streak += 1
        else:
            streak = 0
        if seen % 20 == 0:
            sys.stderr.write(
                f"  {seen} topics scanned, {len(new_msgs)} new msgs, "
                f"streak {streak}, latest topic {recent}\n"
            )
        if streak >= STOP_STREAK:
            sys.stderr.write(f"  stop streak reached at topic {seen} ({recent})\n")
            break

    if not new_msgs:
        sys.stderr.write("no new messages; archive already current\n")
        return

    out = sorted(new_msgs.values(), key=lambda m: m["msg_num"])
    with open(ARCHIVE, "a") as f:
        for m in out:
            f.write(json.dumps(m, ensure_ascii=False) + "\n")

    nums = [m["msg_num"] for m in out]
    gaps = [
        (a, b) for a, b in zip(range(nums[0], nums[-1]), range(nums[0] + 1, nums[-1] + 1))
        if a not in new_msgs and a > cutoff
    ]
    missing = [n for n in range(nums[0], nums[-1] + 1) if n not in new_msgs]
    sys.stderr.write(
        f"appended {len(out)} messages, msg_num {nums[0]}..{nums[-1]}\n"
    )
    if missing:
        sys.stderr.write(
            f"  gaps (deleted/moderated, absent from API): {missing}\n"
        )


if __name__ == "__main__":
    main()
