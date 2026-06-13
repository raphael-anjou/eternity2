// Export a rendered board <svg> as a standalone SVG file or a rasterized PNG.
// Both work purely client-side off the live DOM node, so whatever the viewer
// shows (conflicts, numbers, coordinates) is exactly what gets saved.
//
// The board SVG references its motif artwork via <use href="#e2m-N">, but those
// <defs> live in a separate global <svg> (see MotifDefs). A standalone export
// must carry that artwork with it, so we resolve every referenced symbol from
// the document and inline a <defs> into the clone — otherwise only the
// background rect survives and the board comes out blank.

function inlineReferencedDefs(clone: SVGSVGElement) {
  const ids = new Set<string>();
  clone.querySelectorAll("use").forEach((u) => {
    const href = u.getAttribute("href") ?? u.getAttribute("xlink:href");
    const m = href?.match(/^#(.+)$/);
    if (m) ids.add(m[1]);
  });
  if (!ids.size) return;

  const defs = document.createElementNS("http://www.w3.org/2000/svg", "defs");
  const seen = new Set<string>();
  const add = (id: string) => {
    if (seen.has(id)) return;
    seen.add(id);
    const node = document.getElementById(id);
    if (!node) return;
    const dup = node.cloneNode(true) as Element;
    defs.appendChild(dup);
    // Pull in anything the node itself references (e.g. clipPath via url(#…)).
    dup.querySelectorAll("[clip-path],[fill],[mask],[filter]").forEach((el) => {
      for (const attr of ["clip-path", "fill", "mask", "filter"]) {
        const ref = el.getAttribute(attr)?.match(/url\(#(.+?)\)/);
        if (ref) add(ref[1]);
      }
    });
  };
  ids.forEach(add);
  clone.insertBefore(defs, clone.firstChild);
}

function serialize(svg: SVGSVGElement): string {
  const clone = svg.cloneNode(true) as SVGSVGElement;
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  clone.setAttribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
  // Inline the viewBox-driven aspect ratio at a fixed pixel size so the file is
  // self-contained (the live node is width:100% of its flex parent).
  const vb = svg.viewBox.baseVal;
  if (vb && vb.width) {
    clone.setAttribute("width", String(vb.width));
    clone.setAttribute("height", String(vb.height));
  }
  inlineReferencedDefs(clone);
  return new XMLSerializer().serializeToString(clone);
}

/** Stable 8-char hex hash (FNV-1a) — identical boards collide, edits don't. */
function shortHash(s: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return (h >>> 0).toString(16).padStart(8, "0");
}

function triggerDownload(href: string, filename: string) {
  const a = document.createElement("a");
  a.href = href;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
}

export function downloadSvg(svg: SVGSVGElement, name: string) {
  const data = serialize(svg);
  const blob = new Blob([data], { type: "image/svg+xml;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  triggerDownload(url, `${name}-${shortHash(data)}.svg`);
  URL.revokeObjectURL(url);
}

export async function downloadPng(svg: SVGSVGElement, name: string, scale = 2) {
  const vb = svg.viewBox.baseVal;
  const w = (vb?.width || svg.clientWidth) * scale;
  const h = (vb?.height || svg.clientHeight) * scale;
  const data = serialize(svg);
  const blob = new Blob([data], { type: "image/svg+xml;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  try {
    const img = new Image();
    await new Promise<void>((resolve, reject) => {
      img.onload = () => resolve();
      img.onerror = () => reject(new Error("svg load failed"));
      img.src = url;
    });
    const canvas = document.createElement("canvas");
    canvas.width = w;
    canvas.height = h;
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("no 2d context");
    ctx.drawImage(img, 0, 0, w, h);
    await new Promise<void>((resolve) =>
      canvas.toBlob((png) => {
        if (png) {
          const pngUrl = URL.createObjectURL(png);
          triggerDownload(pngUrl, `${name}-${shortHash(data)}.png`);
          URL.revokeObjectURL(pngUrl);
        }
        resolve();
      }, "image/png"),
    );
  } finally {
    URL.revokeObjectURL(url);
  }
}
