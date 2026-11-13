// Extracted original builder(s): F3. Shared dependencies are in shared/reference.js.
function F3(i, e = {}) {
  let t = new jn(i),
    n = t.M("steel_dark"),
    s = t.M("paint_yellow"),
    o = t.M("steel_galv"),
    r = e.rails || "both",
    a = r === "both" ? [-1, 1] : r === "left" ? [-1] : r === "right" ? [1] : [],
    l = 0.05;
  for (let u = -0.51; u <= 0.511; u += 0.034)
    t.add(o, je($e(2.98, 0.03, 0.004), 0, l - 0.015, u));
  for (let u = -1.45; u <= 1.451; u += 0.1)
    t.add(o, je($e(0.006, 0.006, 1.04), u, l - 0.004, 0));
  for (let u of [-1, 1])
    (t.add(o, je($e(2.99, 0.032, 0.005), 0, l - 0.016, u * 0.525)),
      t.add(o, je($e(0.005, 0.032, 1.05), u * 1.493, l - 0.016, 0)));
  for (let u of [-1, 1])
    t.box(n, 3, 0.18, 0.06, 0, l - 0.09, u * 0.565, { r: 0.008 });
  for (let u of [-1.45, -0.5, 0.5, 1.45])
    t.box(n, 0.05, 0.1, 1.08, u, l - 0.085, 0, { r: 0.006 });
  let c = [
    { min: [-1.5, l - 0.08, -0.6], max: [1.5, l, 0.6], surface: "metal" },
  ];
  for (let u of a) {
    let h = u * 0.565;
    t.box(n, 3, 0.1, 0.008, 0, l + 0.05, u * 0.535, { r: 0 });
    for (let f of [-1.42, 0, 1.42])
      (t.box(s, 0.045, 1.07, 0.045, f, l + 0.52, h, { r: 0.006 }),
        t.box(n, 0.09, 0.012, 0.09, f, l + 0.006, h, { r: 0 }));
    (t.bar(s, [-1.5, l + 1.05, h], [1.5, l + 1.05, h], 0.021, {
      seg: 7,
      caps: !0,
    }),
      t.bar(s, [-1.5, l + 0.55, h], [1.5, l + 0.55, h], 0.016, {
        seg: 6,
        caps: !0,
      }),
      c.push({
        min: [-1.5, l, h - 0.035],
        max: [1.5, l + 1.05, h + 0.035],
        surface: "metal",
      }));
  }
  return qi(t.group(), "catwalk", b1("catwalk"), c);
}
