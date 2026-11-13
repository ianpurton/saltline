// Extracted original builder(s): I3. Shared dependencies are in shared/reference.js.
function I3(i, e = {}) {
  let t = new jn(i),
    n = (e.seed | 0) * 1.7,
    s = t.M("paint_yellow", { vertexColors: !0 }),
    o = t.M("steel_galv"),
    r = 0.083,
    a = (u) => (u > 0.6 && u < 0.69) || (u > 0.78 && u < 0.87),
    l = [
      0.012, 0.3, 0.6, 0.6001, 0.69, 0.6901, 0.78, 0.7801, 0.87, 0.8701, 0.955,
    ];
  for (let u = 0; u < l.length - 1; u++) {
    let h = l[u],
      f = l[u + 1];
    if (f - h < 0.001) continue;
    let p = a((h + f) / 2);
    t.cyl(s, r, r, f - h, 0, (h + f) / 2, 0, {
      seg: 14,
      open: !0,
      col: (d, v, g) => {
        if (p) return [0.05, 0.05, 0.05];
        let x =
          (0.55 + 0.45 * mn(0, 0.35, v)) *
          (0.9 + 0.2 * ri(d * 30 + n, v * 6 + g * 30));
        return [x, x * 0.97, x * 0.9];
      },
    });
  }
  let c = mr(r, 14, 5, hn * 2, 0, hn / 2);
  (c.scale(1, 0.55, 1),
    t.add(s, je(c, 0, 0.955, 0), [0.95, 0.93, 0.88]),
    t.cyl(o, 0.085, 0.085, 0.012, 0, 0.006, 0, { seg: 4, rot: [0, hn / 4, 0] }),
    t.box(o, 0.17, 0.012, 0.17, 0, 0.006, 0, { r: 0.004 }));
  for (let u of [-1, 1])
    for (let h of [-1, 1])
      t.cyl(o, 0.011, 0.011, 0.012, u * 0.066, 0.017, h * 0.066, { seg: 6 });
  return (
    t.cyl(s, r + 0.006, r + 0.006, 0.02, 0, 0.022, 0, {
      seg: 14,
      open: !0,
      col: [0.5, 0.48, 0.42],
    }),
    qi(t.group(), "bollard_post", ca("bollard_post"), [
      { min: [-0.085, 0, -0.085], max: [0.085, 1, 0.085], surface: "metal" },
    ])
  );
}
