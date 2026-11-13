// Extracted original builder(s): qA. Shared dependencies are in shared/reference.js.
function qA(i, e) {
  let t = new Kn(i, "crate_stack", e.seed || 0),
    n = t.rng;
  E1(t, 1.2, 0.95, 1, { label: "HEAVY", code: "SLT 04" + (t.seed % 2) });
  let s = {
    x: n.range(-0.1, 0.02),
    z: n.range(-0.05, 0.05),
    th: (n() < 0.5 ? -1 : 1) * n.range(0.1, 0.16),
    L: 0.86,
    W: 0.72,
  };
  (t.push({ p: [s.x, 0.95, s.z], r: [0, s.th, 0] }),
    E1(t, s.L, 0.6, s.W, {
      detail: !1,
      skids: !1,
      label: "FRAGILE",
      code: "B4-204",
    }),
    t.pop());
  let o = {
    x: s.x + n.range(-0.05, 0.05),
    z: s.z + n.range(-0.04, 0.04),
    th: n.range(-0.12, 0.12),
  };
  (t.push({ p: [o.x, 1.55, o.z], r: [0, o.th, 0] }),
    E1(t, 0.55, 0.35, 0.45, { detail: !1, skids: !1, label: "", code: "" }),
    t.pop());
  let r = (a, l, c, u, h) => {
    let f = (a - l.x - h * u * Math.sin(c)) / Math.cos(c);
    return l.z - f * Math.sin(c) + h * u * Math.cos(c);
  };
  for (let a of [-0.2, 0.17]) {
    let c = [],
      u = (h) => [
        [a, 0.09, h * 0.506],
        [a, 0.956, h * 0.506],
        [a, 1.556, r(a, s, s.th, s.W / 2, h) + h * 0.006],
        [a, 1.906, r(a, o, o.th, 0.225, h) + h * 0.006],
      ];
    (c.push(...u(1), ...u(-1).reverse()),
      t.add("paint_orange", vr(c, [0.02, 0, 0]), null, [1, 0.8, 0.5]),
      t.add(
        "steel_worn",
        Ge(0.06, 0.09, 0.025, 0.006),
        { p: [a, 0.62, 0.5 + 0.016] },
        0.8,
      ),
      t.add(
        "steel_dark",
        vn([a - 0.035, 0.66, 0.53], [a + 0.035, 0.66, 0.53], 0.008, 6, !0),
        null,
        0.8,
      ));
  }
  return t.finish();
}
