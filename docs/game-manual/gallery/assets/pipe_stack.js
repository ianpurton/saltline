// Extracted original builder(s): KA. Shared dependencies are in shared/reference.js.
function KA(i, e) {
  let t = new Kn(i, "pipe_stack", e.seed || 0),
    n = t.rng,
    s = 0.1365,
    o = 0.012,
    r = 0.08,
    a = 14,
    l = (h = 1) => {
      let f = n.range(0.6, 0.95) * h;
      return [f, f * 0.95, f * 0.86];
    };
  for (let h of [-2.2, 0, 2.2]) {
    t.add(
      "wood_dark",
      Ge(0.11, r, 1.2, 0.008),
      {
        p: [h + n.range(-0.1, 0.1), r / 2, 0],
        r: [0, n.range(-0.03, 0.03), 0],
      },
      l(),
    );
    for (let f of [1, -1])
      t.add(
        "wood_pallet",
        Mo(
          f > 0
            ? [
                [0, 0],
                [0.158, 0],
                [0.158, 0.1],
              ]
            : [
                [-0.158, 0],
                [0, 0],
                [-0.158, 0.1],
              ],
          0.1,
          { soft: 0 },
        ),
        { p: [h, r, f * 0.44], r: [0, -tt, 0] },
        l(0.9),
      );
  }
  let c = [4, 3, 2],
    u = 2 * s * Math.sin(At / 3);
  c.forEach((h, f) => {
    for (let p = 0; p < h; p++) {
      let d = (p - (h - 1) / 2) * 2 * s,
        v = r + s + f * u,
        g = n.range(5.78, 5.92),
        x = n.range(-3, 3 - g) * 0.999 + 0,
        y = Math.max(-3, Math.min(3 - g, x)) + g / 2,
        b = n() < 0.7 ? "steel_rust" : "steel_dark",
        m = n() * 50,
        A = n.range(0.75, 1),
        _ = new lt.CylinderGeometry(s, s, g, a, 4, !0);
      {
        let w = _.attributes.uv,
          S = _.attributes.position;
        for (let z = 0; z < w.count; z++)
          w.setXY(z, w.getX(z) * 2 * At * s, S.getY(z));
      }
      let M = { p: [y, v, d], r: [n() * 6, 0, tt] };
      t.add(
        b,
        _,
        M,
        (w, S, z) => A * (0.75 + 0.35 * gn(w * 1.3 + m, z * 9, S * 9)),
      );
      let E = M1(ot(s - o, s - o, g - 0.002, a, !0));
      t.add(
        "steel_rust",
        E,
        M,
        (w) =>
          0.16 +
          0.5 *
            j3(
              g / 2 + Math.abs(y) - 0.9,
              g / 2 + Math.abs(y) + 0.2,
              Math.abs(w - y) + Math.abs(y),
            ),
      );
      for (let w of [1, -1])
        t.add(
          b,
          us(
            w > 0
              ? [
                  [s, 0],
                  [s - o, 0],
                ]
              : [
                  [s - o, 0],
                  [s, 0],
                ],
            a,
          ),
          { p: [y + (w * g) / 2, v, d], r: [0, 0, -tt] },
          A * 1.1,
        );
    }
  });
  for (let h of [-1.35, 1.5]) {
    let p = 2 * s,
      d = r + s + 2 * u + s + 0.004,
      v = [],
      g = [
        [-(1.5 * p + s + 0.004), r + s],
        [-(0.5 * p + s * 0.5 + 0.004), r + s + 2 * u + s * 0.87],
        [-0.5 * p, d],
        [0.5 * p, d],
        [0.5 * p + s * 0.5 + 0.004, r + s + 2 * u + s * 0.87],
        [1.5 * p + s + 0.004, r + s],
        [1.5 * p, r - 0.004],
        [-1.5 * p, r - 0.004],
        [-(1.5 * p + s + 0.004), r + s],
      ];
    for (let x of g) v.push([h, x[1], x[0]]);
    (t.add("steel_dark", vr(v, [0.016, 0, 0]), null, 0.7),
      t.add(
        "steel_galv",
        Ge(0.04, 0.012, 0.05, 0.003),
        { p: [h, d + 0.005, 0.03] },
        0.8,
      ));
  }
  return (
    t.cdecal(
      i.mats.label("API 5L X52  273.1 x 12.5  HEAT 40719", {
        color: "white",
        font: "stencil",
        distress: 0.5,
      }),
      s + 0.002,
      0.2,
      { p: [0.9, r + s + 2 * u, s], r: [-tt + 0.9, 0, tt] },
      1.6,
    ),
    t.finish()
  );
}
