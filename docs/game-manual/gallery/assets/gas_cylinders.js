// Extracted original builder(s): JA. Shared dependencies are in shared/reference.js.
function JA(i, e) {
  let t = new Kn(i, "gas_cylinders", e.seed || 0),
    n = t.rng,
    s = 12,
    o = () => n.range(0.6, 0.9);
  t.add("diamond_plate", Ge(0.9, 0.016, 0.5, 0.004, "bottom"), null, 0.8);
  for (let u of [-0.43, 0.43])
    (t.add(
      "steel_dark",
      Ge(0.04, 1.14, 0.04, 0.005),
      { p: [u, 0.585, -0.225] },
      o(),
    ),
      t.add(
        "steel_dark",
        Ge(0.036, 0.036, 0.43, 0.005),
        { p: [u, 1.06, -0.01] },
        o(),
      ),
      t.add(
        "steel_dark",
        Ge(0.036, 0.036, 0.43, 0.005),
        { p: [u, 0.3, -0.01] },
        o(),
      ),
      t.add(
        "steel_dark",
        Ge(0.036, 0.32, 0.036, 0.005),
        { p: [u, 0.16, 0.19] },
        o(),
      ),
      t.add(
        "steel_galv",
        ja(0.016, 0.004, 4, 8),
        { p: [u - Math.sign(u) * 0, 1.06, 0.215], r: [0, tt, 0] },
        0.8,
      ));
  for (let u of [0.45, 1.06])
    t.add(
      "steel_dark",
      Ge(0.9, 0.04, 0.036, 0.005),
      { p: [0, u, -0.225] },
      o(),
    );
  (t.add(
    "paint_yellow",
    Ge(0.3, 0.2, 0.004, 0.001),
    { p: [0, 0.76, -0.245] },
    0.9,
  ),
    t.decal(i.mats.decal("hazard_flammable"), 0.15, {
      p: [0, 0.76, -0.2475],
      r: [0, At, 0],
    }));
  let r = [
      ["paint_red", 1],
      ["paint_white", 1],
      ["paint_blue", 1],
      ["paint_yellow", 1],
      ["paint_grey", 0.5],
    ],
    a = [
      ["paint_grey", 0.75],
      ["paint_grey", 0.55],
      ["paint_red", [0.55, 0.4, 0.4]],
      ["paint_blue", 0.8],
    ];
  for (let u = 0; u < 3; u++) {
    let h = (u - 1) * 0.272,
      f = 1 - n.range(0, 0.05),
      p = 0.115,
      d = r[(t.seed + u * 2) % r.length],
      v = a[(t.seed + u) % a.length],
      g = u * 5.3 + t.seed,
      x = n() * 6;
    (t.push({
      p: [h, 0.016, 0.015 + n.range(-0.01, 0.01)],
      r: [0, x, 0],
      s: [1, f, 1],
    }),
      t.add(
        v[0],
        us(
          [
            [0, 0.012],
            [0.095, 0.012],
            [0.095, 0.012],
            [0.1, 0],
            [0.111, 0],
            [p, 0.008],
            [p, 0.4],
            [p, 0.8],
            [p, 1.22],
          ],
          s,
        ),
        null,
        (y, b, m) => {
          let A = gn(Math.atan2(m, y) * 3 + g, b * 5),
            _ = A > 0.7 ? 0.55 : 1,
            M = typeof v[1] == "number" ? [v[1], v[1], v[1]] : v[1];
          return [
            M[0] * _,
            M[1] * _ * (A > 0.7 ? 0.85 : 1),
            M[2] * _ * (A > 0.7 ? 0.7 : 1),
          ];
        },
      ),
      t.add(
        d[0],
        us(
          [
            [p, 1.22],
            [0.111, 1.285],
            [0.098, 1.34],
            [0.075, 1.385],
            [0.052, 1.41],
            [0.04, 1.42],
          ],
          s,
        ),
        null,
        d[1],
      ),
      t.add(
        "steel_dark",
        us(
          [
            [0.04, 1.42],
            [0.057, 1.42],
            [0.057, 1.42],
            [0.057, 1.448],
            [0.057, 1.448],
            [0.05, 1.452],
            [0.053, 1.53],
            [0.046, 1.565],
            [0.026, 1.585],
            [0, 1.588],
          ],
          s,
        ),
        null,
        0.8,
      ));
    for (let y of [0, At])
      t.add(
        "polymer_black",
        ot(0.016, 0.016, 0.004, 6),
        { p: [Math.cos(y) * 0.052, 1.5, Math.sin(y) * 0.052], r: [0, -y, tt] },
        0.2,
      );
    (u !== 1 &&
      t.cdecal(
        i.mats.decal(u ? "hazard_general" : "hazard_flammable"),
        p + 0.0015,
        0.1,
        { p: [0, 1.08, 0], r: [0, n.range(-0.5, 0.5) - x, 0] },
      ),
      t.cdecal(
        i.mats.label(
          ["ACETYLENE", "OXYGEN", "ARGON", "PROPANE"][(t.seed + u) % 4],
          { color: "white", font: "stencil", distress: 0.4 },
        ),
        p + 0.0015,
        0.2,
        { p: [0, 0.86, 0], r: [0, n.range(-0.4, 0.4) - x, 0] },
      ),
      t.pop());
  }
  let l = 19,
    c = (u) => {
      let h = 0.215;
      for (let f = -1; f <= 1; f++) {
        let p = u - f * 0.272;
        Math.abs(p) < 0.115 &&
          (h = Math.max(h, 0.015 + Math.sqrt(0.123 * 0.123 - p * p) + 0));
      }
      return Math.max(h, 0.1);
    };
  for (let u = 0; u < l; u++) {
    let h = u / (l - 1),
      f = -0.425 + 0.85 * h,
      p = Math.max(c(f), c(f - 0.03), c(f + 0.03)) + 0.004,
      d = 1.05 - 0.035 * Math.sin(At * ((h * 3) % 1)) - 0.01;
    t.add(
      "steel_galv",
      ja(0.017, 0.0045, 3, 6),
      { p: [f, d, p], r: [u % 2 ? tt : 0.15, 0, 0], s: [1.35, 1, 1] },
      n.range(0.6, 0.95),
    );
  }
  return t.finish();
}
