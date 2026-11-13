// Extracted original builder(s): $A. Shared dependencies are in shared/reference.js.
function $A(i, e) {
  let t = new Kn(i, "tire_stack", e.seed || 0),
    n = t.rng,
    s = zi(e.count | 0 || 4, 1, 5);
  for (let o = 0; o < s; o++) {
    let r = tv(0.39, 0.25, 0.215, s > 4 ? 16 : 20),
      a = {
        p: [
          o ? n.range(-0.022, 0.022) : 0,
          o * 0.25 - (o ? 0.004 : 0),
          o ? n.range(-0.022, 0.022) : 0,
        ],
        r: [0, n() * 6, 0],
      },
      l = n.range(0.75, 1),
      c = o * 3.1;
    t.add("tire", r, a, (u, h, f) => {
      let p = Math.hypot(u - a.p[0], f - a.p[2]);
      return (
        l * (p < 0.23 ? 0.5 : 0.8 + 0.2 * gn(Math.atan2(f, u) * 3 + c, p * 30))
      );
    });
  }
  return t.finish({ size: [0.8, s * 0.25, 0.8] });
}
