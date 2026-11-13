// Extracted original builder(s): XA. Shared dependencies are in shared/reference.js.
function XA(i, e) {
  let t = new Kn(i, "ibc_tote", e.seed || 0),
    n = t.rng,
    s = () => n.range(0.75, 1);
  for (let y of [-0.44, 0, 0.44])
    t.add("steel_galv", $e(1.2, 0.028, 0.1), { p: [0, 0.014, y] }, s());
  for (let y of [-0.54, 0, 0.54])
    for (let b of [-0.44, 0, 0.44])
      t.add(
        "steel_galv",
        Ge(0.12, 0.074, 0.1, 0.008),
        { p: [y, 0.065, b] },
        s() * 0.85,
      );
  for (let y of [-0.48, 0.48])
    t.add("steel_galv", Ge(1.2, 0.03, 0.04, 0.006), { p: [0, 0.115, y] }, s());
  for (let y of [-0.58, 0.58])
    t.add("steel_galv", Ge(0.04, 0.03, 0.92, 0.006), { p: [y, 0.115, 0] }, s());
  t.add("steel_galv", $e(1.12, 0.012, 0.92), { p: [0, 0.124, 0] }, 0.6);
  let o = 1.1,
    r = 0.95,
    a = 0.9,
    l = 0.13 + r / 2,
    c = eo(o, r, a, 0.07, 8, 6, 8);
  (to(c, (y) => {
    let b = y.x / (o / 2),
      m = y.y / (r / 2),
      A = y.z / (a / 2),
      _ = Math.cos(tt * m) * Math.cos(tt * A),
      M = Math.cos(tt * b) * Math.cos(tt * A),
      E = Math.cos(tt * b) * Math.cos(tt * m);
    (Math.abs(b) > 0.9 &&
      (y.x +=
        Math.sign(b) *
        (0.016 * _ + 0.004 * Math.abs(Math.sin((y.y * At) / 0.22)) * _)),
      Math.abs(A) > 0.9 &&
        (y.z +=
          Math.sign(A) *
          (0.016 * E + 0.004 * Math.abs(Math.sin((y.y * At) / 0.22)) * E)),
      m > 0.9 &&
        (y.y +=
          0.012 * M - 0.01 * Math.max(0, 1 - Math.hypot(y.x, y.z) / 0.3)));
  }),
    gr(c));
  let u = 0.13 + r * n.range(0.45, 0.8),
    h = t.seed * 1.3,
    f = t.mat("plastic_grey", { color: 14278096 });
  (t.add(f, c, { p: [0, l, 0] }, (y, b, m) => {
    let A = j3(u - 0.03, u + 0.03, b + 0.02 * gn(y * 3 + h, m * 3)),
      _ = 0.9 + 0.1 * gn(y * 6, b * 6 + h, m * 6);
    return [_ * (0.52 + 0.48 * A), _ * (0.6 + 0.4 * A), _ * (0.66 + 0.34 * A)];
  }),
    t.add(f, ot(0.12, 0.125, 0.03, 14), { p: [0, 0.13 + r + 0.005, 0] }, 0.9),
    t.add(
      "plastic_red",
      us(
        [
          [0.105, 0],
          [0.105, 0.042],
          [0.105, 0.042],
          [0.09, 0.05],
          [0, 0.05],
        ],
        14,
      ),
      { p: [0, 0.13 + r + 0.015, 0] },
      0.9,
    ));
  for (let y = 0; y < 8; y++) {
    let b = (y * At) / 4;
    t.add(
      "plastic_red",
      $e(0.012, 0.04, 0.02),
      {
        p: [Math.cos(b) * 0.108, 0.13 + r + 0.036, Math.sin(b) * 0.108],
        r: [0, -b, 0],
      },
      0.8,
    );
  }
  let p = 0.575,
    d = 0.475,
    v = 0.13,
    g = 1.1,
    x = 0.011;
  for (let y of [0.2, 0.42, 0.64, 0.86, g]) {
    let b = y === g ? 0.014 : x;
    for (let m of [1, -1])
      (t.add("steel_galv", vn([-p, y, m * d], [p, y, m * d], b, 5), null, s()),
        t.add(
          "steel_galv",
          vn([m * p, y, -d], [m * p, y, d], b, 5),
          null,
          s(),
        ));
  }
  for (let y of [1, -1]) {
    for (let b = -0.46; b <= 0.461; b += 0.184)
      t.add(
        "steel_galv",
        vn([b, v, y * (d - 0.012)], [b, g, y * (d - 0.012)], x, 5),
        null,
        s(),
      );
    for (let b = -0.36; b <= 0.361; b += 0.18)
      t.add(
        "steel_galv",
        vn([y * (p - 0.012), v, b], [y * (p - 0.012), g, b], x, 5),
        null,
        s(),
      );
    t.add(
      "steel_galv",
      vn([y * 0.28, g + 0.01, -d], [y * 0.28, g + 0.01, d], 0.013, 5),
      null,
      s(),
    );
  }
  return (
    t.add(
      "aluminium",
      Ge(0.42, 0.3, 0.004, 0.0015),
      { p: [-0.1, 0.72, d + 0.014] },
      0.85,
    ),
    t.label(
      "IBC 1000 L",
      { color: "black", font: "sans", distress: 0.2 },
      0.36,
      0.06,
      { p: [-0.1, 0.8, d + 0.0175] },
    ),
    t.decal(
      i.mats.decal(["hazard_flammable", "hazard_general"][t.seed % 2]),
      0.15,
      { p: [-0.19, 0.67, d + 0.0175] },
    ),
    t.decal(i.mats.decal("numbers_0_9"), 0.2, { p: [0, 0.66, d + 0.0175] }),
    t.decal(i.mats.decal("leak_stain"), 0.3, {
      p: [p + 0.004, 0.3, 0.02],
      r: w1.px,
    }),
    t.add(
      "polymer_black",
      vn([0.52, 0.2, 0], [0.575, 0.2, 0], 0.05, 10, !0),
      null,
      1,
    ),
    t.add(
      "plastic_red",
      vn([0.575, 0.2, 0], [0.598, 0.2, 0], 0.045, 10, !0),
      null,
      0.9,
    ),
    t.add(
      "plastic_red",
      Ge(0.02, 0.022, 0.13, 0.005),
      { p: [0.55, 0.262, 0.02], r: [0, 0.4, 0] },
      0.9,
    ),
    t.add("polymer_black", ot(0.012, 0.012, 0.03, 6), { p: [0.55, 0.245, 0] }),
    t.finish()
  );
}
