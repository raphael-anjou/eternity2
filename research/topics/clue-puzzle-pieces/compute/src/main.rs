//! Solve and verify a recovered Eternity II clue puzzle.
//!
//! Reads a clue-puzzle piece file (`ID T R B L` per line, edge order URDL =
//! up/right/down/left, colour 0 = grey border), encodes "place every piece so
//! all edges match and grey sits exactly on the rim" as SAT, solves it with
//! `kissat`, then INDEPENDENTLY validates the model (all pieces used once, every
//! internal edge matched, grey only on the rim). Emits either a JSON record
//! (default) or a self-contained SVG of the solution (`--svg`).
//!
//! SAT is used rather than a hand-rolled backtracker because it is decisive and
//! fast on these instances (a naive row-major backtracker thrashes on Clue 1).
//! Requires `kissat` on PATH.
//!
//! Args: <pieces_file> <W> <H> [--svg]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

type Edge = u16;

fn rot(e: [Edge; 4], k: u8) -> [Edge; 4] {
    // clockwise quarter turns on URDL edges
    let mut a = e;
    for _ in 0..(k % 4) {
        a = [a[3], a[0], a[1], a[2]];
    }
    a
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let w: usize = args[2].parse().unwrap();
    let h: usize = args[3].parse().unwrap();
    let want_svg = args.iter().any(|a| a == "--svg");
    let n = w * h;

    let text = fs::read_to_string(path).unwrap();
    let mut pieces: Vec<[Edge; 4]> = Vec::new();
    for line in text.lines() {
        let nums: Vec<Edge> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        let e = match nums.len() {
            5 => [nums[1], nums[2], nums[3], nums[4]],
            4 => [nums[0], nums[1], nums[2], nums[3]],
            _ => continue,
        };
        pieces.push(e);
    }
    assert_eq!(pieces.len(), n, "piece count {} != {w}x{h}", pieces.len());

    // Compact the colour labels to a dense 0..k range, keeping 0 = grey border.
    // Source files number colours arbitrarily (SHORTER uses 0..7; jwortmann's
    // clue3/4 leave gaps and go up to 22), which only matters for palette lookup;
    // remapping keeps the solve identical and the render inside the palette.
    {
        let mut remap: HashMap<u16, u16> = HashMap::new();
        remap.insert(0, 0);
        let mut next: u16 = 1;
        for p in &pieces {
            for &c in p {
                remap.entry(c).or_insert_with(|| {
                    let v = next;
                    next += 1;
                    v
                });
            }
        }
        for p in &mut pieces {
            for c in p.iter_mut() {
                *c = remap[c];
            }
        }
    }

    // Candidate (cell, piece, rot) placements that respect the grey-on-rim rule.
    // Variable numbering: 1-based, assigned on first use.
    let mut var: HashMap<(usize, usize, u8), i32> = HashMap::new();
    let mut nvars = 0i32;
    let mut vid = |c: usize, p: usize, r: u8, nvars: &mut i32| -> i32 {
        *var.entry((c, p, r)).or_insert_with(|| {
            *nvars += 1;
            *nvars
        })
    };

    let outward = |x: usize, y: usize| -> [bool; 4] {
        // URDL: is this side facing off the board?
        [y == 0, x == w - 1, y == h - 1, x == 0]
    };

    // Per-cell candidate list.
    let mut cand: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];
    for c in 0..n {
        let (x, y) = (c % w, c / w);
        let ob = outward(x, y);
        for p in 0..n {
            for r in 0..4u8 {
                let e = rot(pieces[p], r);
                let mut ok = true;
                for s in 0..4 {
                    if ob[s] && e[s] != 0 {
                        ok = false;
                    }
                    if !ob[s] && e[s] == 0 {
                        ok = false;
                    }
                }
                if ok {
                    cand[c].push((p, r));
                }
            }
        }
    }

    let mut clauses: Vec<Vec<i32>> = Vec::new();

    // Each cell holds exactly one candidate.
    for c in 0..n {
        let lits: Vec<i32> = cand[c].iter().map(|&(p, r)| vid(c, p, r, &mut nvars)).collect();
        if lits.is_empty() {
            eprintln!("cell {c} has no legal candidate; unsatisfiable");
            std::process::exit(1);
        }
        clauses.push(lits.clone());
        for i in 0..lits.len() {
            for j in (i + 1)..lits.len() {
                clauses.push(vec![-lits[i], -lits[j]]);
            }
        }
    }

    // Each piece used exactly once.
    let mut slots: Vec<Vec<i32>> = vec![Vec::new(); n];
    for c in 0..n {
        for &(p, r) in &cand[c] {
            let v = vid(c, p, r, &mut nvars);
            slots[p].push(v);
        }
    }
    for p in 0..n {
        let lits = &slots[p];
        clauses.push(lits.clone());
        for i in 0..lits.len() {
            for j in (i + 1)..lits.len() {
                clauses.push(vec![-lits[i], -lits[j]]);
            }
        }
    }

    // Edge matching: forbid mismatching adjacent candidate pairs.
    for y in 0..h {
        for x in 0..w {
            let c = y * w + x;
            if x + 1 < w {
                let c2 = c + 1;
                for &(p, r) in &cand[c] {
                    let rc = rot(pieces[p], r)[1];
                    for &(p2, r2) in &cand[c2] {
                        if p2 == p {
                            continue;
                        }
                        if rc != rot(pieces[p2], r2)[3] {
                            clauses.push(vec![-vid(c, p, r, &mut nvars), -vid(c2, p2, r2, &mut nvars)]);
                        }
                    }
                }
            }
            if y + 1 < h {
                let c2 = c + w;
                for &(p, r) in &cand[c] {
                    let bc = rot(pieces[p], r)[2];
                    for &(p2, r2) in &cand[c2] {
                        if p2 == p {
                            continue;
                        }
                        if bc != rot(pieces[p2], r2)[0] {
                            clauses.push(vec![-vid(c, p, r, &mut nvars), -vid(c2, p2, r2, &mut nvars)]);
                        }
                    }
                }
            }
        }
    }

    // Write DIMACS and solve with kissat.
    let cnf_path = std::env::temp_dir().join(format!("cluesat_{w}x{h}.cnf"));
    {
        let mut f = fs::File::create(&cnf_path).unwrap();
        writeln!(f, "p cnf {} {}", nvars, clauses.len()).unwrap();
        for cl in &clauses {
            for l in cl {
                write!(f, "{l} ").unwrap();
            }
            writeln!(f, "0").unwrap();
        }
    }
    let out = Command::new("kissat")
        .arg(cnf_path.to_str().unwrap())
        .stdout(Stdio::piped())
        .output()
        .expect("kissat not found on PATH");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut model: Vec<i32> = Vec::new();
    let mut sat = false;
    for line in stdout.lines() {
        if line.starts_with("s ") {
            sat = line.contains("SATISFIABLE") && !line.contains("UNSAT");
        }
        if line.starts_with("v ") {
            for tok in line[2..].split_whitespace() {
                if let Ok(v) = tok.parse::<i32>() {
                    model.push(v);
                }
            }
        }
    }
    if !sat {
        eprintln!("kissat: UNSATISFIABLE for {path}");
        std::process::exit(1);
    }
    let truth: std::collections::HashSet<i32> = model.iter().filter(|&&v| v > 0).copied().collect();

    // Decode: cell -> edges.
    let inv: HashMap<i32, (usize, usize, u8)> = var.iter().map(|(&k, &v)| (v, k)).collect();
    let mut grid = vec![[0u16; 4]; n];
    let mut filled = vec![false; n];
    for &v in &truth {
        if let Some(&(c, p, r)) = inv.get(&v) {
            grid[c] = rot(pieces[p], r);
            filled[c] = true;
        }
    }

    // Independent validation.
    let mut ok = filled.iter().all(|&b| b);
    let mut assigned = vec![false; n];
    for cell in &grid {
        let mut found = false;
        for (i, &pe) in pieces.iter().enumerate() {
            if assigned[i] {
                continue;
            }
            if (0..4).any(|k| rot(pe, k) == *cell) {
                assigned[i] = true;
                found = true;
                break;
            }
        }
        if !found {
            ok = false;
        }
    }
    let mut matched = 0u32;
    for y in 0..h {
        for x in 0..w {
            let c = y * w + x;
            let g = grid[c];
            if x + 1 < w {
                if g[1] != grid[c + 1][3] {
                    ok = false;
                } else if g[1] != 0 {
                    matched += 1;
                }
            }
            if y + 1 < h {
                if g[2] != grid[c + w][0] {
                    ok = false;
                } else if g[2] != 0 {
                    matched += 1;
                }
            }
            if (x == 0) != (g[3] == 0) || (x == w - 1) != (g[1] == 0) {
                ok = false;
            }
            if (y == 0) != (g[0] == 0) || (y == h - 1) != (g[2] == 0) {
                ok = false;
            }
        }
    }
    let internal = (2 * w * h - w - h) as u32;

    if want_svg {
        print!("{}", render_svg(&grid, w, h));
        return;
    }

    println!("{{");
    println!("  \"file\": \"{path}\",");
    println!("  \"width\": {w}, \"height\": {h}, \"pieces\": {n},");
    println!("  \"solver\": \"kissat\",");
    println!("  \"solved\": true,");
    println!("  \"validated\": {ok},");
    println!("  \"matchedInternalEdges\": {matched}, \"internalEdges\": {internal},");
    print!("  \"solutionURDL\": [");
    for (k, c) in grid.iter().enumerate() {
        if k > 0 {
            print!(", ");
        }
        print!("[{},{},{},{}]", c[0], c[1], c[2], c[3]);
    }
    println!("]");
    println!("}}");
}

/// Render the solved board as a self-contained SVG (our own renderer, no viewer).
fn render_svg(grid: &[[u16; 4]], w: usize, h: usize) -> String {
    // Index 0 = grey border; the rest are distinct motif hues. The first nine are
    // unchanged from the Clue 1/2 renders; the tail covers Clue 3/4 (up to nine
    // non-grey colours each) after the colour labels are compacted to 0..k.
    const PAL: [&str; 13] = [
        "#9a9a9a", "#d62828", "#2176dd", "#2eb85c", "#f2c744", "#8e44c9", "#f0821e", "#17c3c8",
        "#e65aa8", "#7d5a3c", "#b0c94a", "#3b3f8f", "#e0e0e0",
    ];
    let cell = 40i32;
    let (pw, ph) = (w as i32 * cell, h as i32 * cell);
    let col = |c: u16| PAL.get(c as usize).copied().unwrap_or("#ffffff");
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {pw} {ph}\" width=\"{pw}\" height=\"{ph}\" role=\"img\">\n<rect width=\"100%\" height=\"100%\" fill=\"#111\"/>\n"
    );
    for (k, e) in grid.iter().enumerate() {
        let (x, y) = ((k % w) as i32, (k / w) as i32);
        let (x0, y0) = (x * cell, y * cell);
        let (x1, y1) = (x0 + cell, y0 + cell);
        let (mx, my) = (x0 + cell / 2, y0 + cell / 2);
        let mut tri = |ax: i32, ay: i32, bx: i32, by: i32, c: u16| {
            s.push_str(&format!(
                "<polygon points=\"{ax},{ay} {bx},{by} {mx},{my}\" fill=\"{}\"/>\n",
                col(c)
            ));
        };
        tri(x0, y0, x1, y0, e[0]);
        tri(x1, y0, x1, y1, e[1]);
        tri(x0, y1, x1, y1, e[2]);
        tri(x0, y0, x0, y1, e[3]);
        s.push_str(&format!(
            "<rect x=\"{x0}\" y=\"{y0}\" width=\"{cell}\" height=\"{cell}\" fill=\"none\" stroke=\"#1c1c1c\" stroke-width=\"1\"/>\n"
        ));
    }
    s.push_str("</svg>\n");
    s
}
