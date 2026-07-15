//! in-process ONNX inference for `ValueOrder::Learned`.
//!
//! Loads an ONNX model at solver construction (path from env var
//! `E2_LEARNED_MODEL`, default `ml/runs/v1/model.onnx`) and runs
//! inference via the `ort` crate. The sidecar `.meta.json` next to the
//! `.onnx` file selects between two model schemas:
//! - `version: 1` (or missing) — fixed-size, fixed-piece-count
//!     model. Single forward gives one logit per `piece_id * 4 + rot`.
//! - `version: 2` — position-relative model. Takes nb_idx +
//!     valid_mask + candidate edges; emits one score per candidate.
//!     Works on any grid size / piece count.
//!
//! On startup failure (missing file, invalid ONNX, missing inputs) the
//! scorer reports unavailable and the engine falls back to insertion
//! order — preserves correctness for tests.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;
use std::sync::Mutex;

use ndarray::{Array1, Array2, Array3};
use ort::session::Session;
use ort::value::Tensor;

pub enum LearnedScorer {
 /// layout. Fixed grid size + fixed piece count baked in.
    V1 {
        session: Mutex<Session>,
        grid_size: usize,
        n_pieces: u32,
    },
 /// layout. Size + piece-count agnostic.
    V2 {
        session: Mutex<Session>,
    },
}

impl LearnedScorer {
    pub fn spawn() -> Option<Self> {
        let path = std::env::var("E2_LEARNED_MODEL")
            .unwrap_or_else(|_| "ml/runs/v1/model.onnx".to_string());
        Self::from_path(&path)
    }

    fn from_path(path: &str) -> Option<Self> {
        let p = PathBuf::from(path);
        if !p.exists() {
            return None;
        }
        let session = Session::builder().ok()?.commit_from_file(&p).ok()?;

        let meta = read_meta(path);
        let version = meta.as_ref().map(|m| m.version).unwrap_or(1);

        if version == 2 {
            Some(Self::V2 { session: Mutex::new(session) })
        } else {
            let (grid_size, n_pieces) = meta
                .map(|m| (m.size.unwrap_or(6), m.n_pieces.unwrap_or(36)))
                .unwrap_or((6, 36));
            Some(Self::V1 { session: Mutex::new(session), grid_size, n_pieces })
        }
    }

 /// entry point. Used when the engine builds the v1 feature
    /// tensor + candidates. v2 models accept the same input on the
    /// feats/target_pos side but need the extra nb_idx/valid_mask/cand
    /// tensors — for v2, callers should use `score_v2` instead.
    pub fn score(
        &self,
        feats: &Array3<f32>,
        target_pos: u32,
        candidates: &[(u16, u8)],
    ) -> Option<Vec<f32>> {
        match self {
            Self::V1 { session, grid_size, n_pieces } => {
                let n_cells = grid_size * grid_size;
                if feats.shape() != [1, n_cells, 13] {
                    return None;
                }
                let mut sess = session.lock().ok()?;
                let feats_t = Tensor::from_array(feats.clone()).ok()?;
                let tp_t = Tensor::from_array(
                    Array1::from_vec(vec![i64::from(target_pos)]),
                ).ok()?;
                let outputs = sess.run(ort::inputs![
                    "feats" => feats_t,
                    "target_pos" => tp_t,
                ]).ok()?;
                let (shape, data) = outputs[0].try_extract_tensor::<f32>().ok()?;
                if shape.len() != 2 || shape[0] != 1 {
                    return None;
                }
                let n_actions = shape[1] as usize;
                if n_actions != *n_pieces as usize * 4 {
                    return None;
                }
                let mut scores = Vec::with_capacity(candidates.len());
                for &(pid, rot) in candidates {
                    if u32::from(pid) >= *n_pieces || rot >= 4 {
                        scores.push(f32::NEG_INFINITY);
                        continue;
                    }
                    let action = usize::from(pid) * 4 + usize::from(rot);
                    scores.push(data[action]);
                }
                Some(scores)
            }
            Self::V2 { .. } => None, // wrong entry point for v2
        }
    }

 /// entry point — position-relative scoring. The engine
    /// constructs (cell_feats, nb_idx, valid_mask, target_pos,
    /// cand_edges) directly. Works on any grid size; the model has no
    /// fixed-size baked-in axes.
    pub fn score_v2(
        &self,
        feats: &Array3<f32>,        // (1, n_cells, 13)
        nb_idx: &Array2<i64>,       // (n_cells, 4)
        valid_mask: &Array2<f32>,   // (n_cells, 4)
        target_pos: u32,
        cand_edges: &Array3<i64>,   // (1, n_cands, 4)
    ) -> Option<Vec<f32>> {
        let session = match self {
            Self::V2 { session } => session,
            Self::V1 { .. } => return None,
        };
        let mut sess = session.lock().ok()?;
        let feats_t = Tensor::from_array(feats.clone()).ok()?;
        let nb_t = Tensor::from_array(nb_idx.clone()).ok()?;
        let vm_t = Tensor::from_array(valid_mask.clone()).ok()?;
        let tp_t = Tensor::from_array(Array1::from_vec(vec![i64::from(target_pos)])).ok()?;
        let cand_t = Tensor::from_array(cand_edges.clone()).ok()?;
        let outputs = sess.run(ort::inputs![
            "feats" => feats_t,
            "nb_idx" => nb_t,
            "valid_mask" => vm_t,
            "target_pos" => tp_t,
            "cand_edges" => cand_t,
        ]).ok()?;
        let (shape, data) = outputs[0].try_extract_tensor::<f32>().ok()?;
        if shape.len() != 2 || shape[0] != 1 {
            return None;
        }
        let n_cands = shape[1] as usize;
        Some(data[..n_cands].to_vec())
    }

    /// For V1 only: the trained grid size; engine uses it to gate inference.
    /// V2 returns None (no fixed size).
    pub fn grid_size(&self) -> Option<usize> {
        match self {
            Self::V1 { grid_size, .. } => Some(*grid_size),
            Self::V2 { .. } => None,
        }
    }

    pub fn is_v2(&self) -> bool {
        matches!(self, Self::V2 { .. })
    }
}

#[derive(serde::Deserialize)]
struct Meta {
    #[serde(default = "default_version")]
    version: u32,
    size: Option<usize>,
    n_pieces: Option<u32>,
}

fn default_version() -> u32 { 1 }

fn read_meta(model_path: &str) -> Option<Meta> {
    let raw = std::fs::read_to_string(format!("{model_path}.meta.json")).ok()?;
    serde_json::from_str(&raw).ok()
}
