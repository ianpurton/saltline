// Extracted original builder(s): P3. Shared dependencies are in shared/reference.js.
function P3(i, e = {}) {
  let t = new jn(i),
    n = (e.seed | 0) * 2.1,
    s = t.M("steel_dark", { vertexColors: !0 }),
    o = t.M("steel_rust"),
    r = (a, l, c) => {
      let u = mn(0.3, 0.4, l) * mn(0.47, 0.4, l),
        h = 0.75 + 0.4 * ri(a * 6 + n, c * 6 + l * 4) + u * 0.9;
      return [h, h * 0.97, h * 0.93];
    };
  (t.defCol.set(s, r),
    t.cyl(s, 0.34, 0.35, 0.045, 0, 0.0225, 0, { seg: 20 }),
    t.cyl(o, 0.37, 0.37, 0.006, 0, 0.003, 0, { seg: 20 }));
  for (let a = 0; a < 6; a++) {
    let l = (a * hn) / 3 + 0.3;
    (t.cyl(
      o,
      0.026,
      0.026,
      0.03,
      Math.cos(l) * 0.285,
      0.058,
      Math.sin(l) * 0.285,
      { seg: 6, flat: !0 },
    ),
      t.cyl(
        o,
        0.013,
        0.013,
        0.022,
        Math.cos(l) * 0.285,
        0.08,
        Math.sin(l) * 0.285,
        { seg: 6 },
      ));
  }
  (t.add(
    s,
    g1(
      [
        [0.25, 0.045],
        [0.21, 0.08],
        [0.165, 0.16],
        [0.145, 0.27],
        [0.15, 0.36],
        [0.17, 0.43],
        [0.19, 0.5],
        [0.17, 0.57],
        [0.1, 0.6],
        [0, 0.602],
      ],
      18,
    ),
  ),
    t.cyl(s, 0.105, 0.115, 0.5, 0, 0.485, 0, {
      seg: 14,
      open: !0,
      rot: [0, 0, hn / 2],
    }));
  for (let a of [-1, 1]) {
    let l = mr(0.108, 14, 8);
    (l.scale(0.55, 1, 1),
      t.add(s, je(l, a * 0.25, 0.485, 0)),
      t.cyl(s, 0.125, 0.125, 0.03, a * 0.235, 0.485, 0, {
        seg: 14,
        rot: [0, 0, hn / 2],
      }));
  }
  return qi(t.group(), "bollard_mooring", ca("bollard_mooring"), [
    { min: [-0.3, 0, -0.22], max: [0.3, 0.6, 0.22], surface: "metal" },
  ]);
}
