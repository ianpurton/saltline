// Extracted original builder(s): U3. Shared dependencies are in shared/reference.js.
function U3(i, e = {}) {
  let t = new jn(i),
    n = e.seed | 0,
    s = $a(n + 21),
    o = t.M("paint_blue", { vertexColors: !0 }),
    r = t.M("paint_orange"),
    a = t.M("steel_galv"),
    l = {
      wood: t.M("wood_pallet"),
      card: t.M("cardboard"),
      tape: t.M("polymer_tan"),
      poly: t.M("polymer_black"),
      wrap: t.M("glass_dirty"),
      drum: t.M("plastic_blue"),
      drum2: t.M("paint_red"),
      dark: t.M("steel_dark"),
    },
    c = 4.2,
    u = 1.305,
    h = 0.5,
    f = [0.04, 0.04, 0.05];
  for (let v of [-1, 1]) {
    for (let x of [-1, 1]) {
      (t.box(o, 0.09, c, 0.07, v * u, c / 2, x * h, {
        r: 0.008,
        col: (y, b) => {
          let m = 0.7 + 0.3 * mn(0, 0.5, b);
          return [m, m, m];
        },
      }),
        t.box(a, 0.14, 0.01, 0.12, v * u, 0.005, x * h, { r: 0 }));
      for (let y of [-0.05, 0.05])
        t.cyl(a, 0.01, 0.01, 0.02, v * u + y, 0.016, x * h, { seg: 6 });
      for (let y = 0.12; y < c - 0.05; y += 0.1)
        for (let b of [-0.022, 0.022])
          t.add(
            o,
            je(cn(0.014, 0.034), v * u + b, y, x * (h + 0.0356), [
              0,
              x > 0 ? 0 : wi,
              0,
            ]),
            f,
          );
    }
    let g = [0.2, 1.1, 2, 2.9, 3.8];
    for (let x = 0; x < g.length; x++)
      (t.box(o, 0.03, 0.03, 2 * h - 0.07, v * u, g[x], 0, { r: 0 }),
        x < g.length - 1 &&
          t.beam(
            o,
            [v * u, g[x], x % 2 ? h - 0.04 : -h + 0.04],
            [v * u, g[x + 1], x % 2 ? -h + 0.04 : h - 0.04],
            0.03,
            0.03,
            { r: 0 },
          ));
    t.box(t.M("paint_yellow"), 0.14, 0.4, 0.02, v * u, 0.2, h + 0.06, {
      r: 0.006,
    });
  }
  let p = [1.45, 2.86];
  for (let v of p) {
    for (let g of [-1, 1]) {
      t.box(r, 2 * u - 0.09, 0.12, 0.05, 0, v, g * (h + 0.01), { r: 0.008 });
      for (let x of [-1, 1])
        t.box(
          r,
          0.012,
          0.2,
          0.065,
          x * (u - 0.051),
          v - 0.02,
          g * (h + 0.005),
          { r: 0 },
        );
    }
    for (let g of [-0.85, 0, 0.85])
      t.add(a, je($e(0.04, 0.025, 2 * h), g, v + 0.045, 0));
    for (let g = -0.45; g <= 0.451; g += 0.1)
      t.add(a, je($e(2 * u - 0.1, 0.005, 0.005), 0, v + 0.062, g));
    for (let g = -1.2; g <= 1.201; g += 0.2)
      t.add(a, je($e(0.005, 0.005, 2 * h + 0.04), g, v + 0.066, 0));
  }
  (t.decal(
    Mi(i, "BAY " + String.fromCharCode(65 + (n % 6)) + "-" + (10 + (n % 40)), {
      color: "black",
      font: "sans",
      distress: 0.1,
    }),
    0.07,
    0,
    p[0],
    h + 0.0365,
    null,
    0.34,
  ),
    t.decal(
      Mi(i, "MAX 2000 KG / LEVEL", {
        color: "black",
        font: "sans",
        distress: 0.1,
      }),
      0.045,
      0.6,
      p[1],
      h + 0.0365,
      null,
      0.5,
    ));
  let d = [
    "cartons",
    "wrapped",
    "drums",
    "cartons",
    "wrapped",
    "pallets",
    "none",
  ];
  return (
    [0, p[0] + 0.068, p[1] + 0.068].forEach((v, g) => {
      for (let x of [-1, 1]) {
        let y = d[Math.floor(s() * d.length)];
        (g === 2 && y === "drums" && (y = "wrapped"),
          y !== "none" &&
            DA(
              t,
              l,
              y,
              x * 0.615 + (s() - 0.5) * 0.03,
              v,
              (s() - 0.5) * 0.05,
              s,
            ));
      }
    }),
    qi(t.group(), "shelf_rack", b1("shelf_rack"), [
      { min: [-1.35, 0, -0.55], max: [1.35, 4.2, 0.55], surface: "metal" },
    ])
  );
}
