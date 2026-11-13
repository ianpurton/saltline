// Extracted original builder(s): z3. Shared dependencies are in shared/reference.js.
function z3(i, e = {}) {
  let t = new jn(i),
    n = e.seed | 0,
    s = t.M("steel_galv"),
    o = t.M("aluminium"),
    r = t.M("paint_white"),
    a = t.M("paint_yellow"),
    l = Jg.includes(e.sign) ? e.sign : Jg[n % Jg.length];
  (t.box(t.M("concrete"), 0.3, 0.07, 0.3, 0, 0.03, 0, { r: 0.015 }),
    t.box(s, 0.05, 2.38, 0.05, 0, 1.2, 0, { r: 0.006 }),
    t.box(s, 0.058, 0.012, 0.058, 0, 2.395, 0, { r: 0.004 }),
    t.box(s, 0.13, 0.012, 0.13, 0, 0.07, 0, { r: 0.003 }));
  let c = ((n % 5) - 2) * 0.006;
  (t.push(_o(0, 0, 0, [0, 0, c])),
    t.box(o, 0.6, 0.6, 0.004, 0, 2.02, 0.031, { r: 0.0015 }),
    t.box(l.startsWith("hazard") ? a : r, 0.58, 0.58, 0.002, 0, 2.02, 0.034, {
      r: 8e-4,
    }),
    t.decal(Jn(i, l), 0.5, 0, 2.02, 0.0362, null, 0.5),
    t.box(o, 0.6, 0.22, 0.004, 0, 1.56, 0.031, { r: 0.0015 }),
    t.box(r, 0.58, 0.2, 0.002, 0, 1.56, 0.034, { r: 8e-4 }),
    t.decal(
      Mi(i, e.text || E3[n % E3.length], {
        color: "black",
        font: "sans",
        distress: 0.15,
      }),
      0.07,
      0,
      1.56,
      0.0362,
      null,
      0.52,
    ));
  for (let u of [1.85, 2.2, 1.56])
    (t.box(s, 0.5, 0.03, 0.012, 0, u, 0.024, { r: 0 }),
      t.box(s, 0.09, 0.04, 0.07, 0, u, -0.002, { r: 0.004 }));
  for (let [u, h] of [
    [-0.26, 2.28],
    [0.26, 2.28],
    [-0.26, 1.76],
    [0.26, 1.76],
  ])
    t.cyl(s, 0.008, 0.008, 0.004, u, h, 0.0365, {
      seg: 6,
      rot: [hn / 2, 0, 0],
    });
  return (
    t.pop(),
    qi(t.group(), "sign_post", ca("sign_post"), [
      { min: [-0.03, 0, -0.03], max: [0.03, 2.4, 0.03], surface: "metal" },
      {
        min: [-0.3, 1.44, 0.02],
        max: [0.3, 2.33, 0.04],
        surface: "metal",
        bulletOnly: !0,
      },
    ])
  );
}
