// Extracted original builder(s): S3. Shared dependencies are in shared/reference.js.
function S3(i, e = {}) {
  let t = new jn(i),
    n = e.seed | 0,
    s = $a(n + 11),
    o = n * 3.7,
    r = t.M("concrete", { vertexColors: !0 }),
    a = [
      [-0.3, 0],
      [0.3, 0],
      [0.3, 0.076],
      [0.127, 0.33],
      [0.1, 0.57],
      [0.077, 0.79],
      [0.06, 0.81],
      [-0.06, 0.81],
      [-0.077, 0.79],
      [-0.1, 0.57],
      [-0.127, 0.33],
      [-0.3, 0.076],
    ],
    l = [-1.5, -1.47];
  for (let f = -1.25; f <= 1.26; f += 0.25) l.push(f);
  l.push(1.47, 1.5);
  let c = [];
  for (let f = 0; f < 7; f++)
    c.push([
      (s() * 2 - 1) * 1.5,
      s() * 0.8,
      0.08 + s() * 0.14,
      s() < 0.5 ? 1 : -1,
    ]);
  let u = la(
    a,
    l,
    (f, p, d) => {
      let v = Math.abs(d) > 1.49 ? 1 : 0;
      return [d, p - v * 0.022 * (p / 0.81), f * (1 - v * 0.09)];
    },
    { closed: !0, caps: !0 },
  );
  t.add(r, u, (f, p, d) => {
    let v =
        0.86 +
        0.2 * ri(f * 1.3 + o, p * 3 + d * 2) +
        0.08 * ri(f * 7, p * 9 + o),
      g = [v, v * 0.99, v * 0.97];
    g = Ir(g, [v * 0.5, v * 0.48, v * 0.44], mn(0.3, 0, p) * 0.8);
    let x = mn(0.55, 0.8, ri(f * 4.5 + o, 3.3)) * mn(0.25, 0.8, p);
    g = Ir(g, [v * 0.62, v * 0.6, v * 0.56], x * 0.6);
    let y =
      mn(0.6, 0.85, ri(f * 2.1 + o * 2, d * 3)) *
      mn(0.2, 0.33, p) *
      mn(0.62, 0.4, p);
    g = Ir(g, [0.22, 0.22, 0.22], y * 0.65);
    for (let b of c)
      if (Math.sign(d || 1) === b[3]) {
        let m = Math.hypot(f - b[0], p - b[1]) / b[2];
        m < 1 && (g = Ir(g, [1.12, 1.1, 1.05], (1 - m) * 0.7));
      }
    return (
      Math.abs(f) > 1.4 &&
        (g = Ir(g, [1.08, 1.06, 1.02], 0.4 * ri(p * 9, d * 9 + o))),
      g
    );
  });
  let h = [0.05, 0.05, 0.05];
  for (let f of [-1, 1])
    for (let p of [-0.85, 0.85])
      (t.add(
        r,
        je(cn(0.3, 0.058), p, 0.03, f * 0.3008, [0, f > 0 ? 0 : hn, 0]),
        h,
      ),
        t.box(r, 0.34, 0.012, 0.012, p, 0.064, f * 0.3, {
          r: 0.003,
          col: [0.7, 0.7, 0.68],
        }));
  for (let f of [-0.9, 0.9])
    t.tube(
      t.M("steel_rust"),
      [
        [f - 0.05, 0.8, 0],
        [f - 0.04, 0.85, 0],
        [f + 0.04, 0.85, 0],
        [f + 0.05, 0.8, 0],
      ],
      0.008,
      { seg: 5 },
    );
  if (e.stripe) {
    let f = t.M("hazard_stripes");
    t.add(f, je(Nc(2.9, 0.118), 0, 0.8125, 0, [-hn / 2, 0, 0]));
    for (let p of [-1, 1])
      t.add(
        f,
        je(Nc(2.9, 0.2), 0, 0.68, p * 0.0905, [
          p > 0 ? -0.108 : 0.108,
          p > 0 ? 0 : hn,
          0,
        ]),
      );
  }
  return qi(t.group(), "jersey_barrier", ca("jersey_barrier"), [
    { min: [-1.5, 0, -0.3], max: [1.5, 0.33, 0.3], surface: "concrete" },
    { min: [-1.5, 0.33, -0.13], max: [1.5, 0.81, 0.13], surface: "concrete" },
  ]);
}
