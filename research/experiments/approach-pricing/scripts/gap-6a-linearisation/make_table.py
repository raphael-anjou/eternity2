"""results.jsonl -> table_all.md (every run), table_memo.md (minimal encoding, 4x4 and 5x5), summary.md."""
import json, collections, statistics as st
R = [json.loads(l) for l in open("results.jsonl")]
def row(r):
    b = f"{r['size']}x{r['size']} seed {r['seed']}"
    if "rank_full" in r:
        return f"| {b} | {r['size']} | {r['encoding']} | {r['field_name']} | {r['D']} | {r['monomials_all']:.0f} | {r['monomials_nonzero']} | {r['rank_full']:.0f} | {r['nullity']} | {r['forced']}/{r['n']} | {'yes' if r['determined'] else 'no'} | {r['seconds']:.2f} |"
    if r.get("partial"):
        return f"| {b} | {r['size']} | {r['encoding']} | {r['field_name']} | {r['D']} | {r['monomials_all']:.0f} | {r['monomials_nonzero']} | rows of spread <= {r['class_reached']} only | n/a | {r['forced']}/{r['n']} | {'yes' if r['determined'] else 'no'} | {r['seconds']:.2f} |"
    return f"| {b} | {r['size']} | {r['encoding']} | {r['field_name']} | {r['D']} | {r['monomials_all']:.0f} | {r['monomials_nonzero']} | did not fit ({r['aborted']}) | n/a | n/a | not known | {r['seconds']:.2f} |"
head = "| board | size | encoding | field | degree | monomials | monomials not zeroed | rank | nullity | variables forced | determined | seconds |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n"
key = lambda r: (r["size"], r["seed"], r["encoding"], r["field_name"], r["D"])
R.sort(key=key)
open("table_all.md", "w").write(head + "\n".join(row(r) for r in R) + "\n")
open("table_memo.md", "w").write(head + "\n".join(row(r) for r in R if r["encoding"] == "minimal" and r["size"] >= 4) + "\n")
agg = collections.defaultdict(list)
for r in R: agg[(r["size"], r["encoding"], r["field_name"], r["D"])].append(r)
out = "| size | encoding | field | degree | boards | determined | forced, min / median / max | boards with a variable forced beyond degree 1 | median monomials | median rank | median nullity | median stored nonzeros | max seconds |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|\n"
d1 = {(r["size"], r["seed"], r["encoding"], r["field_name"]): r["forced"] for r in R if r["D"] == 1}
for k in sorted(agg):
    v = [r for r in agg[k] if "rank_full" in r]
    if not v:
        out += f"| {k[0]}x{k[0]} | {k[1]} | {k[2]} | {k[3]} | {len(agg[k])} | not known (did not fit) | | | | | | | |\n"; continue
    f = sorted(r["forced"] for r in v)
    beyond = sum(1 for r in v if r["forced"] > d1[(r["size"], r["seed"], r["encoding"], r["field_name"])])
    med = lambda name: f"{st.median(r[name] for r in v):.0f}"
    out += f"| {k[0]}x{k[0]} | {k[1]} | {k[2]} | {k[3]} | {len(v)} | {sum(1 for r in v if r['determined'])} | {f[0]} / {st.median(f):.0f} / {f[-1]} of about {v[0]['n']} | {beyond} | {med('monomials_all')} | {med('rank_full')} | {med('nullity')} | {med('basis_nonzeros')} | {max(r['seconds'] for r in v):.2f} |\n"
open("summary.md", "w").write(out)
print(out)
