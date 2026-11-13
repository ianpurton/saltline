// Extracted original builder(s): k3. Shared dependencies are in shared/reference.js.
function k3(i, e = {}) {
  let t = new jn(i),
    n = t.M("diamond_plate"),
    s = t.M("steel_dark"),
    o = t.M("paint_yellow"),
    r = t.M("steel_galv"),
    a = 12,
    l = 0.2,
    c = 0.2,
    u = -1.5,
    h = 0.5,
    f = u + a * c,
    p = [];
  for (let g = 0; g < a; g++) {
    let x = (g + 1) * l,
      y = u + g * c;
    (t.box(n, 0.25, 0.03, 0.9, y + 0.125, x - 0.015, 0, { r: 0.005 }),
      t.box(o, 0.035, 0.034, 0.9, y + 0.016, x - 0.015, 0, { r: 0.006 }));
    for (let b of [-1, 1])
      t.box(s, 0.2, 0.05, 0.01, y + 0.13, x - 0.05, b * 0.452, { r: 0 });
    p.push({ min: [y, 0, -h], max: [y + c, x, h], surface: "metal" });
  }
  for (let g of [-1, 1]) {
    (t.beam(
      s,
      [u + 0.07, 0.13, g * 0.465],
      [f + 0.04, 2.5, g * 0.465],
      0.24,
      0.014,
      { r: 0.004 },
    ),
      t.box(r, 0.3, 0.012, 0.12, u + 0.06, 0.006, g * 0.465, { r: 0 }));
    for (let x of [-0.08, 0.1])
      t.cyl(r, 0.012, 0.012, 0.02, u + 0.06 + x, 0.02, g * 0.5, { seg: 6 });
  }
  let d = (a + 1) * l;
  (t.box(n, 0.64, 0.035, 0.96, f + 0.3, d - 0.0175, 0, { r: 0.005 }),
    t.box(o, 0.035, 0.038, 0.96, f + 0.016, d - 0.0175, 0, { r: 0.006 }));
  for (let g of [-1, 1])
    t.box(s, 0.66, 0.16, 0.06, f + 0.3, d - 0.115, g * 0.47, { r: 0.008 });
  (t.box(s, 0.06, 0.16, 0.96, 1.47, d - 0.115, 0, { r: 0.008 }),
    t.box(s, 0.06, 0.12, 0.9, f + 0.02, d - 0.1, 0, { r: 0.008 }));
  for (let g of [-1, 1])
    (t.box(s, 0.08, d - 0.19, 0.08, 1.44, (d - 0.19) / 2, g * 0.45, {
      r: 0.008,
    }),
      t.box(r, 0.18, 0.012, 0.18, 1.44, 0.006, g * 0.45, { r: 0 }),
      t.beam(
        s,
        [1.44, 1.5, g * 0.45],
        [f + 0.1, d - 0.2, g * 0.45],
        0.05,
        0.05,
        { r: 0.006 },
      ));
  (t.beam(s, [1.44, 0.15, -0.42], [1.44, 2.2, 0.42], 0.045, 0.008, { r: 0 }),
    t.beam(s, [1.44, 0.15, 0.42], [1.44, 2.2, -0.42], 0.045, 0.008, { r: 0 }),
    t.box(s, 0.06, 0.06, 0.82, 1.44, 1.2, 0, { r: 0.006 }),
    p.push({ min: [f, d - 0.2, -h], max: [1.5, d, h], surface: "metal" }));
  for (let g of [-1, 1])
    p.push({
      min: [1.4, 0, g * 0.45 - 0.04],
      max: [1.48, d - 0.2, g * 0.45 + 0.04],
      surface: "metal",
    });
  let v = (g) => ((g - u) / c) * l + l;
  for (let g of [-1, 1]) {
    let x = g * 0.485,
      y = [
        [u + 0.1, v(u + 0.1)],
        [u + 0.9, v(u + 0.9) - 0.1],
        [u + 1.7, v(u + 1.7) - 0.1],
        [f + 0.03, d],
        [1.47, d],
      ];
    for (let [b, m] of y)
      (t.bar(o, [b, m - 0.12, x], [b, m + 1, x], 0.019, { seg: 7 }),
        t.box(s, 0.07, 0.1, 0.012, b, m - 0.1, x - g * 0.012, { r: 0 }));
    for (let b of [1, 0.52]) {
      let m = b === 1 ? 0.021 : 0.016;
      (t.bar(o, [y[0][0], y[0][1] + b, x], [y[3][0], y[3][1] + b, x], m, {
        seg: 7,
      }),
        t.bar(o, [y[3][0], y[3][1] + b, x], [y[4][0], y[4][1] + b, x], m, {
          seg: 7,
        }),
        t.add(o, je(mr(m * 1.05, 7, 5), y[3][0], y[3][1] + b, x)),
        t.add(o, je(mr(m * 1.05, 7, 5), y[0][0], y[0][1] + b, x)),
        t.add(o, je(mr(m * 1.05, 7, 5), y[4][0], y[4][1] + b, x)));
    }
    (t.box(s, 0.6, 0.1, 0.008, f + 0.3, d + 0.05, g * 0.476, { r: 0 }),
      p.push({
        min: [f, d, x - 0.03],
        max: [1.5, d + 1.02, x + 0.03],
        surface: "metal",
      }));
  }
  return qi(t.group(), "stairs_metal", b1("stairs_metal"), p);
}
