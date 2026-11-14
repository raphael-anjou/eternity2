"""Append one finished run's A22 row to its wave file, with provenance fields.

append_row.py RUN_JSONL WAVE_JSONL BINARY WAVE MACHINE STARTED ENDED GUARD_S
"""
import hashlib
import json
import platform
import subprocess
import sys


def cpu():
    for cmd in (["sysctl", "-n", "machdep.cpu.brand_string"],):
        try:
            return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip()
        except (OSError, subprocess.CalledProcessError):
            pass
    try:
        with open("/proc/cpuinfo") as f:
            for line in f:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return platform.processor()


def main():
    src, dst, binary, wave, machine, started, ended, guard = sys.argv[1:9]
    with open(binary, "rb") as f:
        sha = hashlib.sha256(f.read()).hexdigest()
    try:
        with open(src) as f:
            rows = [json.loads(line) for line in f if line.strip()]
    except OSError:
        rows = []
    if not rows:  # the process died without writing its row: keep a visible error row
        rows = [{"status": "error", "error": "no row written (process killed or crashed)", "source": src}]
    with open(dst, "a") as out:
        for r in rows:
            r.update(
                binary=binary,
                binary_sha256=sha,
                wave=int(wave),
                machine=machine,
                cpu=cpu(),
                os=f"{platform.system()} {platform.machine()}",
                started_at=started,
                ended_at=ended,
                guard_s=int(guard),
            )
            out.write(json.dumps(r) + "\n")


if __name__ == "__main__":
    main()
