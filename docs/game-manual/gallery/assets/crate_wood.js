// Extracted original builder(s): GA. Shared dependencies are in shared/reference.js.
function GA(i, e) {
  let t = new Kn(i, "crate_wood", e.seed || 0);
  return (
    E1(t, 1.1, 1, 0.9, {
      label:
        e.label === void 0
          ? ["FRAGILE", "THIS SIDE UP", "KEEP DRY", "MACHINE PARTS"][t.seed % 4]
          : e.label,
      code: e.code || "B4-" + [117, 204, 318][t.seed % 3],
    }),
    t.finish()
  );
}
