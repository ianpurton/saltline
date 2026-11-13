// Extracted original builder(s): aR. Shared dependencies are in shared/reference.js.
function aR(i, e) {
  let t = new Kn(i, "dumpster", e.seed || 0),
    n = t.rng,
    [s, o] = Ja(e.color, "green"),
    r = (M = 1) => (typeof o == "number" ? o * M : o.map((E) => E * M)),
    a = 0.89,
    l = 0.2,
    c = 1.12,
    u = 0.3,
    h = 0.5,
    f = -0.44,
    p = -0.5,
    d = 0.022,
    v = [
      [-h, c],
      [-u, l],
      [-f, l],
      [-p, c],
      [-p - d, c],
      [-f - d * 0.8, l + d],
      [-u + d * 0.8, l + d],
      [-h + d, c],
    ],
    g = t.seed * 2.3;
  t.add(s, Mo(v, a * 2, { soft: 0 }), { r: [0, tt, 0] }, (M, E, w) => {
    let S =
      Math.abs(w) < 0.4 &&
      E > l + 0.01 &&
      E < c - 0.01 &&
      Math.abs(M) < a - 0.01;
    return r(0.9 + 0.1 * gn(M * 3 + g, E * 3));
  });
  for (let M of [1, -1]) {
    (t.add(
      s,
      Mo(
        [
          [-h, c],
          [-u, l],
          [-f, l],
          [-p, c],
        ],
        d,
        { soft: 0 },
      ),
      { p: [M * (a - d / 2), 0, 0], r: [0, tt, 0] },
      r(0.95),
    ),
      t.add(
        s,
        Ge(0.05, 0.045, h - p + 0.04, 0.008),
        { p: [M * a, c, 0] },
        r(0.9),
      ));
    for (let E of [-0.22, 0.14])
      t.add(
        s,
        Ge(0.03, 0.84, 0.05, 0.006),
        { p: [M * (a + 0.012), 0.66, E] },
        r(0.9),
      );
    (t.add(
      s,
      Ge(0.035, 0.1, 0.84, 0.008),
      { p: [M * (a + 0.014), 0.93, 0] },
      r(0.85),
    ),
      t.add(
        "steel_worn",
        vn(
          [M * (a + 0.01), 0.93, 0.3],
          [M * (a + 0.065), 0.93, 0.3],
          0.03,
          10,
          !0,
        ),
        null,
        0.7,
      ),
      t.add(
        "steel_worn",
        ot(0.042, 0.042, 0.008, 10),
        { p: [M * (a + 0.064), 0.93, 0.3], r: [0, 0, tt] },
        0.6,
      ));
  }
  (t.add(s, Ge(a * 2 + 0.04, 0.045, 0.05, 0.008), { p: [0, c, h] }, r(0.9)),
    t.add(s, Ge(a * 2 + 0.04, 0.045, 0.05, 0.008), { p: [0, c, p] }, r(0.9)));
  let x = Math.atan2(h - u, c - l),
    y = Math.atan2(p - f, c - l);
  for (let M of [-0.6, -0.2, 0.2, 0.6])
    (t.add(
      s,
      zr(
        [M, l + 0.03, u + 0.022],
        [M, c - 0.04, h + 0.012],
        0.055,
        0.03,
        0.006,
      ),
      null,
      r(0.9),
    ),
      t.add(
        s,
        zr(
          [M, l + 0.03, f - 0.016],
          [M, c - 0.04, p - 0.012],
          0.055,
          0.03,
          0.006,
        ),
        null,
        r(0.9),
      ));
  t.add(
    s,
    zr(
      [-a, l + 0.5, u + 0.022 + (0.5 * (h - u)) / (c - l)],
      [a, l + 0.5, u + 0.022 + (0.5 * (h - u)) / (c - l)],
      0.03,
      0.06,
      0.006,
    ),
    { r: [0, 0, 0] },
    r(0.88),
  );
  let b = (M) => u + ((M - l) * (h - u)) / (c - l) + 0.003;
  (t.label(
    e.label || "B4 WASTE " + (11 + (t.seed % 2)),
    { color: "white", font: "stencil", distress: 0.5 },
    0.36,
    0.09,
    { p: [-0.4, 0.93, b(0.93)], r: [-x, 0, 0] },
  ),
    t.decal(i.mats.decal("grime_streak"), 0.36, {
      p: [0.4, 0.82, b(0.82)],
      r: [-x, 0, 0],
    }),
    t.decal(i.mats.decal("rust_run"), 0.3, {
      p: [0, 0.5, b(0.5)],
      r: [-x, 0, 0],
    }),
    t.decal(i.mats.decal("logo_c"), 0.26, {
      p: [0.4, 0.42, b(0.42)],
      r: [-x, 0, 0],
    }),
    t.decal(i.mats.decal("leak_stain"), 0.34, {
      p: [-0.4, 0.45, b(0.45)],
      r: [-x, 0, 0],
    }));
  let m = c + 0.045,
    A = p - 0.02;
  t.add(
    "steel_galv",
    vn([-a - 0.02, m, A], [a + 0.02, m, A], 0.013, 6, !0),
    null,
    0.7,
  );
  let _ = (M, E) => {
    t.push({ p: [M, m, A], r: [E, 0, 0] });
    let w = 1.07,
      S = eo(0.885, 0.04, w, 0.016, 3, 3, 4);
    (to(S, (H) => {
      H.y > 0 &&
        (H.y +=
          0.018 * Math.cos((H.z / w) * At) * Math.cos((H.x / 0.885) * At));
    }),
      S.computeVertexNormals());
    let z = n.range(0.75, 0.95);
    t.add("plastic_grey", S, { p: [0, 0.005, w / 2] }, z);
    for (let H of [-0.26, 0.26])
      t.add(
        "plastic_grey",
        Ge(0.07, 0.016, w - 0.16, 0.006),
        { p: [H, 0.036, w / 2] },
        z * 0.9,
      );
    t.add(
      "plastic_grey",
      Ge(0.3, 0.035, 0.05, 0.01),
      { p: [0, 0.02, w + 0.01] },
      z * 0.8,
    );
    for (let H of [-0.3, 0.3])
      t.add(
        "plastic_grey",
        Ge(0.08, 0.03, 0.09, 0.008),
        { p: [H, 0.005, 0.03] },
        z * 0.8,
      );
    t.pop();
  };
  (_(-0.45, 0), _(0.45, tt + 0.07 + n.range(0, 0.05)));
  for (let [M, E, w, S] of [
    [0.45, 0.93, 0.05, 1],
    [0.66, 0.9, -0.24, 0.8],
    [0.22, 0.86, -0.2, 0.85],
  ]) {
    let z = eo(0.46 * S, 0.4 * S, 0.42 * S, 0.17 * S, 3, 3, 3),
      H = n() * 20;
    (to(z, (O) => {
      let q = gn(O.x * 9 + H, O.y * 9, O.z * 9) - 0.5;
      ((O.x += q * 0.06), (O.z += q * 0.06), (O.y += q * 0.04));
    }),
      t.add(
        "polymer_black",
        gr(z),
        { p: [M, E, w], r: [n() * 0.5, n() * 6, 0] },
        0.8,
      ));
  }
  t.add(
    "cardboard",
    vr(
      [
        [0.2, 0.9, 0.2],
        [0.24, 1.14, 0.26],
        [0.3, 1.2, 0.33],
      ],
      [0.16, 0, -0.05],
    ),
    null,
    0.7,
  );
  for (let M of [-0.7, 0.7])
    for (let E of [-0.3, 0.17]) {
      (t.push({ p: [M, 0, E], r: [0, n() * 6, 0] }),
        t.add(
          "steel_galv",
          Ge(0.13, 0.012, 0.11, 0.003),
          { p: [0, 0.194, 0] },
          0.7,
        ));
      for (let w of [1, -1])
        t.add(
          "steel_galv",
          zr(
            [0, 0.19, w * 0.034],
            [0.035, 0.08, w * 0.034],
            0.05,
            0.006,
            0,
            !0,
          ),
          null,
          0.65,
        );
      (t.add(
        "rubber",
        ot(0.08, 0.08, 0.045, 14),
        { p: [0.035, 0.08, 0], r: [tt, 0, 0] },
        0.9,
      ),
        t.add(
          "steel_galv",
          ot(0.035, 0.035, 0.05, 8),
          { p: [0.035, 0.08, 0], r: [tt, 0, 0] },
          0.6,
        ),
        t.pop());
    }
  return t.finish({
    colliders: [
      { min: [-0.95, 0.05, -0.56], max: [0.95, 1.2, 0.55], surface: "metal" },
    ],
  });
}
