#!/bin/sh
# TOY BOARDS ONLY. Resume check on the 5x5 validation board (5, 3, 4, seed 1), leak objective, rho 300.
# A: 400 iterations uninterrupted. B: 200, stop, resume to 400 (exact checkpoint, with Z and residuals).
# C: same as B but the checkpoint stripped to the old format (v, rho, it, elapsed), as the 6x6 ones are.
# Runs from resume/ on the lab machine; compares the final v and Z of B and C against A.
cd "$(dirname "$0")" || exit 1
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 VECLIB_MAXIMUM_THREADS=1 MKL_NUM_THREADS=1
UV=${UVCMD:-uv}
R="$UV run --with numpy --with scipy python -I admm.py 5 3 4 1 leak 300 100000"
rm -rf vt && mkdir -p vt
ADMM_MAX_IT=400 nice -n 15 $R vt/A > vt/A.out 2>&1
ADMM_MAX_IT=200 nice -n 15 $R vt/B > vt/B.out 2>&1
cp -r vt/B vt/C
python3 -I -c "import json;p='vt/C/ckpt_leak_5_3_4_1.json';m=json.load(open(p));json.dump({k:m[k] for k in ('it','rho','elapsed')},open(p,'w'))"
rm vt/C/ckpt_leak_5_3_4_1_Z.npy
ADMM_MAX_IT=400 nice -n 15 $R vt/B >> vt/B.out 2>&1
ADMM_MAX_IT=400 nice -n 15 $R vt/C >> vt/C.out 2>&1
$UV run --with numpy python -I -c "
import numpy as np, json
for k in ('B','C'):
    for f in ('ckpt_leak_5_3_4_1.npy','ckpt_leak_5_3_4_1_Z.npy'):
        a=np.load('vt/A/'+f); b=np.load('vt/'+k+'/'+f)
        print(k, f, 'max abs diff', float(np.abs(a-b).max()))
    print(k, json.load(open('vt/'+k+'/ckpt_leak_5_3_4_1.json')), 'A', json.load(open('vt/A/ckpt_leak_5_3_4_1.json')))
"
