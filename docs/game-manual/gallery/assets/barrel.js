// Extracted original builder(s): HA. Shared dependencies are in shared/reference.js.
function HA(i, e, t) {
  let n = new Kn(i, "barrel", t),
    s = n.rng,
    o = 22,
    [r, a] = Ja(e.color, "blue");
  e.lying && n.push({ p: [-0.45, 0.297, 0], r: [0, 0, -tt] });
  let l = us(OA(!!e.open), o),
    c = [];
  for (let d = 0; d < 2 + (t % 2); d++)
    c.push([
      s() * At * 2,
      s.range(0.12, 0.78),
      s.range(0.07, 0.13),
      s.range(0.006, 0.014),
    ]);
  (to(l, (d) => {
    let v = Math.hypot(d.x, d.z);
    if (v < 0.27) return;
    let g = Math.atan2(d.z, d.x);
    for (let x of c) {
      let y = Math.abs(g - x[0]) % (At * 2);
      y > At && (y = At * 2 - y);
      let b = Math.hypot(y * 0.285, d.y - x[1]);
      if (b < x[2]) {
        let m = 1 - (x[3] * (0.5 + 0.5 * Math.cos((At * b) / x[2]))) / v;
        ((d.x *= m), (d.z *= m));
      }
    }
  }),
    l.computeVertexNormals());
  let u = l.attributes.normal,
    h = l.attributes.position.count / (o + 1);
  for (let d = 0; d < h; d++) {
    let v = d * (o + 1),
      g = v + o,
      x = u.getX(v) + u.getX(g),
      y = u.getY(v) + u.getY(g),
      b = u.getZ(v) + u.getZ(g),
      m = Math.hypot(x, y, b) || 1;
    (u.setXYZ(v, x / m, y / m, b / m), u.setXYZ(g, x / m, y / m, b / m));
  }
  let f = t * 2.37,
    p = e.color === "rust";
  return (
    n.add(r, l, null, (d, v, g) => {
      let x = Math.hypot(d, g),
        y = Math.atan2(g, d);
      if (x < 0.27) {
        let w = v > 0.5 ? 0.62 + 0.3 * gn(d * 9 + f, g * 9) : 0.6;
        return [w, w, w];
      }
      let b = Math.pow(gn(y * 2.6 + f, 3.1), 2.2),
        m =
          v > 0.6
            ? (0.88 - v) / 0.28
            : v > 0.31
              ? (0.6 - v) / 0.29
              : (0.31 - v) / 0.31,
        A = 1 - 0.55 * b * (1 - 0.6 * zi(m, 0, 1)),
        M = (x > 0.289 ? 1 : 0) * (0.35 + 0.65 * gn(y * 7 + f, v * 20)),
        E = [A * (1 - 0.25 * M), A * (1 - 0.42 * M), A * (1 - 0.55 * M)];
      return (
        typeof a != "number" &&
          ((E[0] *= a[0]), (E[1] *= a[1]), (E[2] *= a[2])),
        E
      );
    }),
    e.open ||
      (n.add(
        "steel_galv",
        ot(0.036, 0.038, 0.016, 8),
        { p: [0.17, 0.891, 0.02] },
        0.8,
      ),
      n.add(
        "steel_galv",
        ot(0.017, 0.018, 0.014, 8),
        { p: [-0.16, 0.89, -0.08] },
        0.8,
      )),
    !p &&
      t % 3 !== 2 &&
      n.cdecal(i.mats.decal("hazard_flammable"), 0.2855, 0.15, {
        p: [0, 0.455, 0],
        r: [0, 0.9 + t, 0],
      }),
    t % 2 === 0 &&
      n.cdecal(
        i.mats.label("UN 1202", {
          color:
            p || e.color === "white" || e.color === "yellow"
              ? "black"
              : "white",
          font: "stencil",
          distress: 0.5,
        }),
        0.2855,
        0.2,
        { p: [0, 0.74, 0], r: [0, 2.6 + t, 0] },
      ),
    n.finish(e.lying ? { size: [0.9, 0.6, 0.6] } : {})
  );
}
