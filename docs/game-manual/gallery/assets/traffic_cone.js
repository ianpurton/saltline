// Extracted original builder(s): jA. Shared dependencies are in shared/reference.js.
function jA(i, e, t) {
  let n = new Kn(i, "traffic_cone", t),
    s = n.rng,
    o = t * 4.1;
  e.lying &&
    (n.push({ p: [-0.34, 0.18, 0], r: [0, s() * 6, 0] }),
    n.push({ r: [0, 0, -tt + 0.16] }));
  let r = (h) => 0.128 + ((0.031 - 0.128) * (h - 0.05)) / (0.685 - 0.05);
  n.add("rubber", Ge(0.36, 0.03, 0.36, 0.008, "bottom"), null, 0.9);
  let a = (h, f, p) => {
      let d = gn(Math.atan2(p, h) * 3 + o, f * 14);
      return d > 0.68 ? 0.45 : 0.85 + 0.15 * d;
    },
    l = (h, f, p = [], d = []) => us([...p, [r(h), h], [r(f), f], ...d], 16),
    c = n.mat("plastic_red", { color: 12734234 }),
    u = n.mat("plastic_grey", { color: 13224386 });
  return (
    n.add(
      c,
      l(0.05, 0.36, [
        [0.165, 0.03],
        [0.15, 0.04],
        [0.136, 0.05],
      ]),
      null,
      a,
    ),
    n.add(u, l(0.36, 0.5), null, (h, f, p) => 0.85 * a(h, f, p)),
    n.add(c, l(0.5, 0.56), null, a),
    n.add(u, l(0.56, 0.61), null, (h, f, p) => 0.85 * a(h, f, p)),
    n.add(
      c,
      l(
        0.61,
        0.685,
        [],
        [
          [0.028, 0.696],
          [0.018, 0.7],
          [0.018, 0.7],
          [0, 0.7],
        ],
      ),
      null,
      a,
    ),
    e.lying && (n.pop(), n.pop()),
    n.finish(e.lying ? { size: [0.72, 0.36, 0.36] } : {})
  );
}
