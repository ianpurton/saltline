// Extracted original builder(s): A3. Shared dependencies are in shared/reference.js.
function A3(i, e = {}) {
  let t = new jn(i),
    n = $a((e.seed | 0) + 5),
    s = t.M("steel_galv"),
    o = t.M("concrete", { vertexColors: !0 }),
    r = t.M("chainlink", { doubleSide: !0 }),
    a = e.posts || "both",
    l = e.lean === 1 ? 1 : -1,
    c =
      a === "both"
        ? [-1.5, 1.5]
        : a === "left"
          ? [-1.5]
          : a === "right"
            ? [1.5]
            : [];
  for (let p of c) {
    (t.box(o, 0.32, 0.09, 0.32, p, 0.035, 0, {
      r: 0.02,
      col: (d, v) => {
        let g = 0.6 + 0.4 * mn(0, 0.08, v);
        return [g, g, g];
      },
    }),
      t.cyl(s, 0.03, 0.03, 2.02, p, 1.05, 0, { seg: 8, open: !0 }),
      t.cyl(s, 0.05, 0.05, 0.012, p, 0.086, 0, { seg: 8 }),
      t.bar(s, [p, 2.02, 0], [p, 2.4 - 0.02, l * 0.36], 0.019, {
        seg: 6,
        caps: !0,
      }),
      t.cyl(s, 0.034, 0.034, 0.05, p, 2.03, 0, { seg: 8 }));
    for (let d of [0.2, 1.05, 1.9])
      t.cyl(s, 0.034, 0.034, 0.03, p, d, 0, { seg: 8, open: !0 });
  }
  t.bar(s, [-1.5, 2, 0.036], [1.5, 2, 0.036], 0.021, { seg: 7 });
  for (let p of [0.09, 1.02])
    t.bar(s, [-1.5, p, 0.012], [1.5, p, 0.012], 0.0035, { seg: 4 });
  (t.box(s, 0.008, 1.9, 0.02, -1.462, 1.03, 0.012, { r: 0 }),
    t.box(s, 0.008, 1.9, 0.02, 1.462, 1.03, 0.012, { r: 0 }));
  let u = new rn(2.92, 1.93, 6, 3),
    h = u.attributes.position,
    f = u.attributes.uv;
  for (let p = 0; p < h.count; p++) {
    let d = h.getX(p),
      v = h.getY(p);
    (f.setXY(p, d + 1.5, v + 1),
      h.setZ(
        p,
        0.012 +
          0.02 *
            (ri(d * 1.3 + (e.seed | 0), v * 1.5) - 0.5) *
            (1 - Math.abs(d) / 1.46),
      ));
  }
  (u.computeVertexNormals(), t.add(r, je(u, 0, 1.025, 0)));
  for (let p = 0; p < 3; p++) {
    let d = 0.3 + p * 0.33,
      v = 2.02 + d * 0.36,
      g = l * d * 0.36;
    t.bar(s, [-1.5, v, g], [1.5, v, g], 0.0032, { seg: 4 });
    for (let x = -1.42 + (p % 2) * 0.09; x < 1.45; x += 0.18)
      t.add(s, je($e(0.004, 0.032, 0.004), x, v, g, [n() * 3, 0, 0.8 + n()]));
  }
  for (let p = 0; p < 5; p++)
    t.add(s, je($e(0.012, 0.012, 0.03), -1.2 + p * 0.6, 2, 0.02));
  return qi(t.group(), "fence_panel", ca("fence_panel"), [
    { min: [-1.5, 0, -0.04], max: [1.5, 2.4, 0.04], surface: "metal" },
  ]);
}
