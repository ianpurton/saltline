// Extracted original builder(s): tR. Shared dependencies are in shared/reference.js.
function tR(i, e) {
  let t = new Kn(i, "sandbags", e.seed || 0),
    n = t.rng,
    s = 0.18,
    o = zi(e.courses | 0 || 5, 1, 7),
    r = t.mat(K3[0], K3[1]),
    a = (l, c, u, h) => {
      let f = s * n.range(1.02, 1.12),
        p = n() < 0.3 ? n.range(0.55, 0.75) : n.range(0.82, 1),
        d = n() * 30,
        v = {
          p: [
            u[0] + n.range(-0.012, 0.012),
            u[1] + f / 2 - 0.004,
            u[2] + n.range(-0.012, 0.012),
          ],
          r: [n.range(-0.03, 0.03), h + n.range(-0.06, 0.06), 0],
        };
      t.add(r, eR(l, f, c, n), v, (g, x, y) => {
        let b = gn(g * 7 + d, x * 7, y * 7);
        return [
          p * (0.85 + 0.15 * b),
          p * (0.84 + 0.15 * b),
          p * (0.8 + 0.16 * b),
        ];
      });
      for (let g of [1, -1])
        t.add(
          r,
          vr(
            [
              [g * (l / 2 - 0.004), n.range(-0.008, 0.008), -c * 0.27],
              [g * (l / 2 - 0.002), n.range(-0.008, 0.008), c * 0.27],
            ],
            [g * 0.009, 0.001, 0],
          ),
          v,
          p * 0.9,
        );
    };
  for (let l = 0; l < o; l++) {
    let c = l * s,
      u = l === o - 1;
    if (l % 2 === 0)
      for (let h = 0; h < 3; h++)
        for (let f of [1, -1])
          (u && h === 2 && f === 1 && o > 2) ||
            a(0.655, 0.3, [(h - 1) * 0.655, c, f * 0.148], 0);
    else
      for (let h = 0; h < 5; h++)
        (u && h === 0 && o > 2) || a(0.585, 0.385, [(h - 2) * 0.392, c, 0], tt);
  }
  return t.finish({ size: [2, o * s, 0.6], surface: "dirt" });
}
