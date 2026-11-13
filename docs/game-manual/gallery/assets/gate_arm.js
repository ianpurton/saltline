// Extracted original builder(s): C3. Shared dependencies are in shared/reference.js.
function C3(i, e = {}) {
  let t = new jn(i),
    n = bs.clamp(e.open || 0, 0, 1),
    s = t.M("paint_orange", { vertexColors: !0 }),
    o = t.M("paint_white"),
    r = t.M("paint_red"),
    a = t.M("steel_dark"),
    l = t.M("steel_galv"),
    c = t.M("paint_yellow"),
    u = -2.5 + 0.19,
    h = 0.92;
  (t.box(t.M("concrete"), 0.5, 0.06, 0.46, u, 0.03, 0, { r: 0.015 }),
    t.box(s, 0.36, 1, 0.3, u, 0.56, 0, {
      r: 0.02,
      col: (y, b) => {
        let m = 0.6 + 0.4 * mn(0.05, 0.5, b);
        return [m, m, m];
      },
    }),
    t.box(a, 0.4, 0.04, 0.34, u, 1.075, 0, { r: 0.012 }),
    t.box(s, 0.3, 0.6, 0.012, u, 0.5, 0.151, {
      r: 0.004,
      col: [0.9, 0.9, 0.9],
    }),
    t.cyl(l, 0.012, 0.012, 0.016, u + 0.1, 0.5, 0.16, {
      seg: 6,
      rot: [hn / 2, 0, 0],
    }),
    t.decal(Jn(i, "hazard_general"), 0.14, u, 0.9, 0.153, null),
    t.decal(Jn(i, "caution_stripes"), 0.08, u, 0.12, 0.153, null, 0.34),
    t.cyl(a, 0.03, 0.03, 0.05, u, 1.12, 0, { seg: 8 }),
    t.add(
      i.mats.emissive(16734750, 2.2),
      je(ot(0.026, 0.026, 0.045, 8), u, 1.165, 0),
      null,
      !0,
    ),
    t.push(_o(u, h, 0, [0, 0, n * 1.48])),
    t.cyl(a, 0.075, 0.075, 0.36, 0, 0, 0, { seg: 10, rot: [hn / 2, 0, 0] }));
  for (let y of [-1, 1])
    t.box(a, 0.62, 0.11, 0.012, 0.05, 0, y * 0.175, { r: 0.004 });
  t.box(a, 0.3, 0.16, 0.3, -0.33, -0.02, 0, { r: 0.02 });
  let f = 4.62,
    p = 0.12,
    d = 9,
    v = f / d;
  for (let y = 0; y < d; y++)
    t.box(
      y % 2 ? o : r,
      v + 0.001,
      0.1 - y * 0.003,
      0.05,
      p + v * (y + 0.5),
      0,
      0,
      { r: 0.008 },
    );
  (t.box(a, 0.03, 0.11, 0.06, p + f + 0.012, 0, 0, { r: 0.008 }),
    t.add(
      i.mats.emissive(16722450, 1.5),
      je($e(0.05, 0.03, 0.054), p + f - 0.5, 0.058, 0),
      null,
      !0,
    ),
    t.pop());
  let g = 2.5 - 0.1;
  (t.box(t.M("concrete"), 0.3, 0.05, 0.3, g, 0.025, 0, { r: 0.012 }),
    t.cyl(c, 0.04, 0.04, 0.8, g, 0.45, 0, { seg: 8 }),
    t.box(c, 0.14, 0.02, 0.16, g, 0.855, 0, { r: 0.005 }));
  for (let y of [-1, 1])
    t.box(c, 0.14, 0.14, 0.014, g, 0.93, y * 0.073, { r: 0.004 });
  t.cyl(l, 0.07, 0.07, 0.01, g, 0.055, 0, { seg: 8 });
  let x = [
    { min: [u - 0.2, 0, -0.17], max: [u + 0.2, 1.1, 0.17], surface: "metal" },
    { min: [g - 0.07, 0, -0.08], max: [g + 0.07, 1, 0.08], surface: "metal" },
  ];
  return (
    n < 0.5 &&
      x.push({
        min: [u, h - 0.07, -0.04],
        max: [g, h + 0.07, 0.04],
        surface: "metal",
      }),
    qi(t.group(), "gate_arm", ca("gate_arm"), x)
  );
}
