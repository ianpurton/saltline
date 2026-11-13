// Extracted original builder(s): YA. Shared dependencies are in shared/reference.js.
function YA(i, e) {
  let t = new Kn(i, "cable_reel", e.seed || 0),
    n = t.rng,
    s = 0.65,
    o = s,
    r = 0.4,
    a = 0.045,
    l = (y = 1) => {
      let b = n.range(0.7, 1) * y;
      return [b, b * n.range(0.93, 1), b * n.range(0.85, 0.97)];
    },
    c = n.range(-0.5, 0.5);
  t.push({ p: [0, o, 0], r: [0, 0, c] });
  for (let y of [1, -1]) {
    for (let b of Z3(s - 0.004, 9, n))
      t.add(
        "wood_dark",
        b,
        { p: [0, 0, y * (r + n.range(0, 0.003))], r: [0, y > 0 ? 0 : At, 0] },
        l(),
      );
    for (let b of Z3(s - 0.004, 9, n))
      t.add(
        "wood_dark",
        b,
        { p: [0, 0, y * (r - a)], r: [0, y > 0 ? At : 0, tt] },
        l(0.8),
      );
    (t.add(
      "wood_dark",
      ot(s, s, a, 24, !0),
      { p: [0, 0, y * (r - a / 2)], r: [tt, 0, 0] },
      l(0.75),
    ),
      t.add(
        "steel_dark",
        ot(0.105, 0.105, 0.008, 12),
        { p: [0, 0, y * (r + 0.006)], r: [tt, 0, 0] },
        0.9,
      ),
      t.add(
        "polymer_black",
        ot(0.04, 0.04, 0.004, 10),
        { p: [0, 0, y * (r + 0.0105)], r: [tt, 0, 0] },
        0.3,
      ));
    for (let b = 0; b < 6; b++) {
      let m = (b * At) / 3 + 0.3;
      t.add(
        "steel_galv",
        ot(0.017, 0.017, 0.014, 6),
        {
          p: [Math.cos(m) * 0.3, Math.sin(m) * 0.3, y * (r + 0.006)],
          r: [tt, 0, 0],
        },
        n.range(0.5, 0.9),
      );
    }
    (t.label(
      y > 0 ? "NYY-J 4x95 mm2  500 m" : "SALTLINE TERMINAL",
      { color: "white", font: "stencil", distress: 0.45 },
      0.7,
      0.085,
      { p: [0, 0.4, y * (r + 0.006)], r: [0, y > 0 ? 0 : At, 0] },
    ),
      y > 0 &&
        t.decal(i.mats.decal("arrow"), 0.2, {
          p: [0.36, -0.2, r + 0.006],
          r: [0, 0, -1.2],
        }));
  }
  let u = 16,
    h = r - a,
    f = (2 * h) / u,
    p = 0.47,
    d = [];
  for (let y = 0; y <= u; y++)
    (d.push([p - 0.017, -h + y * f]), y < u && d.push([p, -h + (y + 0.5) * f]));
  let v = t.seed * 0.7;
  (t.add(
    "rubber",
    us(d, 20),
    { r: [tt, 0, 0] },
    (y, b, m) => 0.7 + 0.5 * gn(Math.atan2(b, y) * 2 + v, m * 30),
  ),
    t.pop());
  let g = 0.2,
    x = [];
  for (let y = 1.9; y >= -0.25; y -= 0.36)
    x.push([
      Math.cos(y) * (p + 0.012),
      o + Math.sin(y) * (p + 0.012),
      g + (1.9 - y) * 0.02,
    ]);
  (x.push(
    [0.53, 0.3, g + 0.05],
    [0.56, 0.08, g + 0.06],
    [0.5, 0.024, g + 0.02],
    [0.3, 0.022, g - 0.1],
    [0.05, 0.022, g - 0.02],
    [-0.2, 0.022, g + 0.06],
    [-0.38, 0.022, g - 0.05],
  ),
    t.add("rubber", Fn(x, 0.02, 6, !1, 0.5), null, 0.9),
    t.add(
      "brass",
      vn([-0.38, 0.022, g - 0.05], [-0.41, 0.022, g - 0.07], 0.014, 6, !0),
      null,
      0.9,
    ));
  for (let y of [r - a / 2, -(r - a / 2)])
    (t.add(
      "wood_pallet",
      Mo(
        [
          [0.14, 0],
          [0.37, 0],
          [0.37, 0.1],
        ],
        0.1,
        { soft: 0 },
      ),
      { p: [0, 0, y] },
      l(0.8),
    ),
      t.add(
        "wood_pallet",
        Mo(
          [
            [-0.37, 0],
            [-0.14, 0],
            [-0.37, 0.1],
          ],
          0.1,
          { soft: 0 },
        ),
        { p: [0, 0, y] },
        l(0.8),
      ));
  return t.finish();
}
