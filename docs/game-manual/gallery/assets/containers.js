// Extracted original builder(s): M3. Shared dependencies are in shared/reference.js.
function M3(i, e, t = {}) {
  let n = _3[e],
    s = t.seed === void 0 ? 0 : Math.abs(Math.floor(t.seed)),
    o = s % IA,
    r = Math.round(bs.clamp(t.doors || 0, 0, 1) * 10) / 10,
    a = Math.round(bs.clamp(t.wear === void 0 ? 0.5 : t.wear, 0, 1) * 2) / 2,
    l = e + "|" + o + "|" + r + "|" + a,
    c = x3.get(i.mats);
  (c || ((c = new Map()), x3.set(i.mats, c)),
    c.has(l) || c.set(l, zA(i, e, o, r, a)));
  let u = c.get(l),
    h = t.color;
  jg.includes(h) ||
    (h = t.seed === void 0 ? n.def : jg[(s * 7 + 3) % jg.length]);
  let f = Zg(
      u.parts,
      new Map([[Jt, i.mats.get("paint_" + h, { vertexColors: !0 })]]),
    ),
    p = Gi[e];
  return (
    (f.userData.color = h),
    (f.userData.doors = r),
    qi(
      f,
      e,
      [p[0], p[1], p[2]],
      u.colliders.map((d) => ({
        min: d.min.slice(),
        max: d.max.slice(),
        surface: d.surface,
      })),
    )
  );
}
