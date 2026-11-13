// Extracted original builder(s): WA. Shared dependencies are in shared/reference.js.
function WA(i, e) {
  let t = new Kn(i, "pallet_stack", e.seed || 0),
    n = t.rng,
    s = zi(e.count | 0 || 8, 1, 12);
  for (let o = 0; o < s; o++) {
    let r = o === s - 1;
    (t.push({
      p: [
        o ? n.range(-0.028, 0.028) : 0,
        o * 0.144,
        o ? n.range(-0.022, 0.022) : 0,
      ],
      r: [
        0,
        n.range(-0.035, 0.035) +
          (n() < 0.3 ? At : 0) +
          (r ? n.range(-0.06, 0.06) : 0),
        0,
      ],
    }),
      S1(t, n, { broken: !r && n() < 0.2, stamp: o % 3 === 0, bevel: r }),
      t.pop());
  }
  return t.finish({ size: [1.2, s * 0.144, 1] });
}
