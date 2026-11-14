"""learning-real-prefixes: one worker = one core. Takes the next unclaimed prefix of QUEUE (claim by mkdir), writes its
CNF, runs the five arms one after the other on this core in an order shuffled by the prefix name, deletes the CNF.
Every run: outer wall and child processor seconds (rusage) appended to out/runs.jsonl, stdout and stderr kept in out/.
Internal limit 1200 s wall in every arm; SIGTERM at 1230 s, SIGKILL at 1290 s as a backstop.
usage: python3 -I worker.py QUEUE LABEL [arm ...]      (pin the worker itself with taskset where it exists)"""
import json, os, random, resource, socket, subprocess, sys, time
HERE = os.path.dirname(os.path.abspath(__file__))
os.chdir(HERE)
LIMIT = int(os.environ.get("LIMIT", "1200"))
UV = os.environ.get("UV", "uv").split()
ARMS = {
    "T": lambda p, c: ["bin/rowscan_real", p, str(LIMIT)],
    "AC": lambda p, c: ["bin/dyn16", "file:" + p, "AC", "ROW", "exact", "100000000000000000", str(LIMIT)],
    "K": lambda p, c: UV + ["run", "-q", "--with", "python-sat==1.9.dev22", "python3", "-I", "cdcl_pysat.py", c, str(LIMIT)],
    "C": lambda p, c: ["bin/clasp", "-q", "--stats=2", "--time-limit=%d" % LIMIT, "-n", "0", c],
    "U0": lambda p, c: ["bin/clasp", "-q", "--stats=2", "--time-limit=%d" % LIMIT, "-n", "0", "--no-lookback", "--lookahead=no", "--heuristic=None", c],
}
queue, label = sys.argv[1], sys.argv[2]
arms = sys.argv[3:] or list(ARMS)
host = socket.gethostname().split(".")[0]
for d in ("out", "claims", "tmp"):
    os.makedirs(d, exist_ok=True)


def log(rec):
    with open("out/runs.jsonl", "a") as f:
        f.write(json.dumps(rec) + "\n")


for pid in open(queue).read().split():
    try:
        os.mkdir("claims/" + pid)
    except FileExistsError:
        continue
    dyn, cnf = "prefixes/%s.dyn" % pid, "tmp/%s.cnf" % pid
    t = time.monotonic()
    enc = subprocess.run([sys.executable, "-I", "enc_real.py", dyn, cnf], capture_output=True, text=True)
    log({"prefix": pid, "arm": "ENC", "host": host, "label": label, "rc": enc.returncode, "wall_ext": round(time.monotonic() - t, 3),
         "info": enc.stdout.strip() + enc.stderr.strip()[-300:], "cnf_bytes": os.path.getsize(cnf) if os.path.exists(cnf) else -1})
    order = list(arms)
    random.Random(pid).shuffle(order)
    for k, arm in enumerate(order):
        cmd = ARMS[arm](dyn, cnf)
        r0, t0, start = resource.getrusage(resource.RUSAGE_CHILDREN), time.monotonic(), time.strftime("%Y-%m-%dT%H:%M:%S")
        killed = 0
        with open("out/%s.%s.out" % (pid, arm), "w") as fo, open("out/%s.%s.err" % (pid, arm), "w") as fe:
            p = subprocess.Popen(cmd, stdout=fo, stderr=fe, stdin=subprocess.DEVNULL)
            try:
                p.wait(timeout=LIMIT + 30)
            except subprocess.TimeoutExpired:
                killed = 1
                p.terminate()
                try:
                    p.wait(timeout=60)
                except subprocess.TimeoutExpired:
                    killed = 2
                    p.kill()
                    p.wait()
        wall = time.monotonic() - t0
        r1 = resource.getrusage(resource.RUSAGE_CHILDREN)
        log({"prefix": pid, "arm": arm, "host": host, "label": label, "pos": k, "start": start, "rc": p.returncode, "killed": killed,
             "wall_ext": round(wall, 3), "cpu_user": round(r1.ru_utime - r0.ru_utime, 3), "cpu_sys": round(r1.ru_stime - r0.ru_stime, 3)})
    if os.path.exists(cnf):
        os.remove(cnf)
    os.makedirs("done", exist_ok=True)
    open("done/" + pid, "w").close()
