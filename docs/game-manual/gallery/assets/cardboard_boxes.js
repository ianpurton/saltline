// Extracted original builder(s): nR. Shared dependencies are in shared/reference.js.
function nR(i, e) {
  let t = new Kn(i, "cardboard_boxes", e.seed || 0),
    n = t.rng,
    s = () => n.range(-0.015, 0.015),
    o = () => n.range(0.82, 1);
  return (
    Oc(
      t,
      0.5,
      0.38,
      0.44,
      { p: [-0.23 + s(), 0, -0.16 + s()], r: [0, n.range(-0.05, 0.05), 0] },
      { damp: 0.45, label: "FRAGILE", arrow: !0, tint: o() },
    ),
    Oc(
      t,
      0.42,
      0.33,
      0.4,
      { p: [0.26 + s(), 0, -0.17 + s()], r: [0, n.range(-0.08, 0.08), 0] },
      { damp: 0.5, label: "KEEP DRY", tint: o() },
    ),
    Oc(
      t,
      0.46,
      0.28,
      0.4,
      { p: [-0.24 + s(), 0.38, -0.15 + s()], r: [0, n.range(-0.2, 0.2), 0] },
      { crush: 1, damp: 0.75, tint: o() * 0.92 },
    ),
    Oc(
      t,
      0.33,
      0.26,
      0.32,
      { p: [0.27 + s(), 0.33, -0.18 + s()], r: [0, n.range(-0.3, 0.3), 0] },
      { open: !0, damp: 0.1, label: "", arrow: !0, tint: o() },
    ),
    Oc(
      t,
      0.3,
      0.16,
      0.24,
      {
        p: [-0.26 + s(), 0.565, -0.16 + s()],
        r: [0.03, n.range(-0.5, 0.5), -0.09],
      },
      { damp: 0.2, label: "HX-2291", tint: o() },
    ),
    Oc(
      t,
      0.36,
      0.26,
      0.3,
      { p: [0.06 + s(), 0.105, 0.2], r: [0.62, n.range(-0.25, 0.25), 0] },
      { damp: 0.9, crush: 0.25, tint: o() * 0.85 },
    ),
    t.finish()
  );
}
