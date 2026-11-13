// Extracted original builder(s): ZA. Shared dependencies are in shared/reference.js.
function ZA(i, e, t) {
  let n = new Kn(i, "jerry_can", t),
    [s, o] = Ja(e.color, "olive"),
    r = eo(0.336, 0.36, 0.156, 0.022, 3, 3, 3, "bottom"),
    a = (x, y) => (y > 0.3 && x > 0.04 ? y - (x - 0.04) * 0.5 : y);
  (to(r, (x) => {
    ((x.y = a(x.x, x.y)),
      Math.abs(x.z) > 0.07 &&
        Math.abs(x.x) < 0.14 &&
        x.y > 0.04 &&
        x.y < 0.3 &&
        (x.z *= 1.02));
  }),
    r.computeVertexNormals(),
    n.add(s, r, null, o),
    n.add(
      s,
      Mo(
        [
          [-0.172, 0],
          [0.172, 0],
          [0.172, 0.297],
          [0.045, 0.364],
          [-0.172, 0.364],
        ],
        0.007,
        { soft: 0 },
      ),
      null,
      typeof o == "number" ? 0.8 : o.map((x) => x * 0.8),
    ));
  let l = 0.24,
    c = 0.2,
    u = -0.025,
    h = 0.165,
    f = Math.hypot(l, c) - 0.02,
    p = Math.atan2(c, l);
  for (let x of [1, -1]) {
    let y = x * 0.079;
    for (let b of [1, -1])
      n.add(s, Ge(l, 0.016, 0.007, 0.003), { p: [u, h + (b * c) / 2, y] }, o);
    for (let b of [1, -1])
      n.add(s, Ge(0.016, c, 0.007, 0.003), { p: [u + (b * l) / 2, h, y] }, o);
    for (let b of [1, -1])
      n.add(
        s,
        Ge(f, 0.018, 0.008, 0.003),
        { p: [u, h, y], r: [0, 0, b * p] },
        o,
      );
  }
  for (let x of [-0.042, 0, 0.042])
    n.add(
      s,
      Fn(
        [
          [-0.135, 0.35, x],
          [-0.125, 0.42, x],
          [-0.09, 0.44, x],
          [-0.02, 0.44, x],
          [0.01, 0.42, x],
          [0.02, 0.35, x],
        ],
        0.0105,
        5,
        !1,
        0.5,
      ),
      null,
      o,
    );
  let d = [0.1, 0.33, 0],
    v = [0.42, 0.9, 0],
    g = (x) => [d[0] + v[0] * x, d[1] + v[1] * x, 0];
  return (
    n.add(s, vn(g(-0.02), g(0.07), 0.03, 8, !0), null, o),
    n.add("steel_dark", vn(g(0.07), g(0.1), 0.036, 8, !0), null, 0.9),
    n.add(
      "steel_galv",
      Ge(0.07, 0.008, 0.02, 0.002),
      { p: g(0.108), r: [0, 0, -0.44] },
      0.8,
    ),
    n.decal(i.mats.decal("hazard_flammable"), 0.055, {
      p: [0.125, 0.25, 0.0795],
    }),
    n.decal(i.mats.decal("hazard_flammable"), 0.055, {
      p: [-0.125, 0.25, -0.0795],
      r: [0, At, 0],
    }),
    n.finish()
  );
}
