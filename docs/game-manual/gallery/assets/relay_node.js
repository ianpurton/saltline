// Extracted original builder(s): B3. Shared dependencies are in shared/reference.js.
function B3(i, e = {}) {
  let t = e.seed | 0,
    n = t * 1.9,
    s = e.id || e.label || "ABC"[t % 3],
    o = new jn(i),
    r = new jn(i),
    a = new jn(i),
    l = new jn(i),
    c = o.M("paint_grey", { vertexColors: !0 }),
    u = o.M("steel_dark"),
    h = o.M("steel_galv"),
    f = o.M("aluminium"),
    p = o.M("rubber"),
    d = o.M("concrete", { vertexColors: !0 }),
    v = (le, R, I) => {
      let se =
        (0.88 + 0.2 * ri(le * 3 + n, R * 3 + I * 2)) *
        (0.6 + 0.4 * mn(0.16, 0.7, R));
      return [se, se, se * 0.98];
    };
  (o.defCol.set(c, v),
    r.defCol.set(c, (le, R, I) => {
      let se = 0.9 + 0.14 * ri(le * 4 + n, R * 3);
      return [se, se, se];
    }));
  let g = [0.035, 0.035, 0.04],
    x = { em: !0 };
  (o.box(d, 1.36, 0.16, 0.96, 0, 0.08, 0, {
    r: 0.025,
    col: (le, R, I) => {
      let se = 0.6 + 0.38 * mn(0, 0.15, R) + 0.1 * ri(le * 5, I * 5);
      return [se, se, se * 0.97];
    },
  }),
    o.decal(Jn(i, "caution_stripes"), 0.1, 0, 0.085, 0.4815, null, 1.2));
  for (let le of [-1, 1])
    for (let R of [-1, 1])
      (o.cyl(h, 0.016, 0.016, 0.03, le * 0.44, 0.185, R * 0.36, { seg: 6 }),
        o.box(h, 0.1, 0.012, 0.1, le * 0.44, 0.166, R * 0.36, { r: 0 }));
  let y = -0.5,
    b = 0.5,
    m = 0.18,
    A = 1.62,
    _ = -0.3,
    M = 0.3,
    E = 0.025,
    w = (m + A) / 2,
    S = A - m;
  o.box(c, 1, S, E, 0, w, _ + E / 2, { r: 0.006 });
  for (let le of [-1, 1])
    o.box(c, E, S, 0.6, le * (0.5 - E / 2), w, 0, { r: 0.006 });
  (o.box(c, 1, E, 0.6, 0, m + E / 2, 0, { r: 0.006 }),
    o.box(c, 1, E, 0.6, 0, A - E / 2, 0, { r: 0.006 }),
    o.box(c, E, S, 0.58, 0, w, 0, { r: 0.004 }),
    o.box(c, 1, 0.12, E, 0, A - 0.06, M - E / 2, { r: 0.006 }),
    o.box(c, 1, 0.07, E, 0, m + 0.035, M - E / 2, { r: 0.006 }),
    o.box(u, 1.04, 0.04, 0.64, 0, m - 0.01, 0, { r: 0.008 }),
    o.box(c, 1.12, 0.05, 0.74, 0, A + 0.025, 0.01, { r: 0.012 }));
  for (let le of [-1, 1])
    o.tube(
      h,
      [
        [le * 0.4 - 0.04, A + 0.05, 0.2],
        [le * 0.4 - 0.03, A + 0.1, 0.2],
        [le * 0.4 + 0.03, A + 0.1, 0.2],
        [le * 0.4 + 0.04, A + 0.05, 0.2],
      ],
      0.008,
      { seg: 5 },
    );
  let z = (le, R, I) => [0.16, 0.16, 0.17];
  (o.add(c, je(cn(0.45, S - 0.06), -0.25, w, _ + E + 0.002), z),
    o.add(c, je(cn(0.55, S - 0.06), y + E + 0.002, w, 0, [0, es / 2, 0]), z),
    o.add(c, je(cn(0.55, S - 0.06), -E / 2 - 0.002, w, 0, [0, -es / 2, 0]), z));
  for (let le = 0; le < 5; le++) {
    let R = 0.42 + le * 0.22;
    (o.box(u, 0.42, 0.17, 0.4, -0.25, R, -0.05, { r: 0.008 }),
      o.box(f, 0.4, 0.02, 0.012, -0.25, R + 0.06, 0.152, { r: 0 }));
    for (let I = 0; I < 4; I++)
      a.add(x, je(cn(0.012, 0.012), -0.41 + I * 0.035, R - 0.04, 0.1512));
    for (let I = 0; I < 3; I++)
      o.add(u, je(pr(0.03, 0.022, 2), -0.2 + I * 0.06, R - 0.03, 0.1512), g);
  }
  let H = [p, p, p];
  for (let le = 0; le < 4; le++)
    o.tube(
      H[le % 3],
      [
        [-0.2 + (le % 3) * 0.06, 0.39 + le * 0.22, 0.15],
        [-0.14 + (le % 3) * 0.05, 0.36 + le * 0.22, 0.24],
        [-0.07, 0.55 + le * 0.22, 0.22],
        [-0.2 + ((le + 1) % 3) * 0.06, 0.61 + le * 0.22, 0.15],
      ],
      0.007,
      { seg: 5 },
    );
  let O = 0.474,
    q = 1.3,
    U = 0.9,
    T = (le, R, I) => {
      let se = R + O / 2,
        ae = 0;
      (le.box(c, O, q, 0.024, se, U, ae, { r: 0.006 }),
        le.box(c, O - 0.07, q - 0.07, 0.006, se, U, ae + 0.013, { r: 0.003 }));
      for (let ve of [U + 0.36, U - 0.4]) {
        le.add(c, je(cn(O - 0.14, 0.3), se, ve, ae + 0.0165), g);
        for (let _e = 0; _e < 7; _e++)
          le.box(
            c,
            O - 0.13,
            0.034,
            0.005,
            se,
            ve - 0.125 + _e * 0.042,
            ae + 0.027,
            { r: 0, rot: [0.75, 0, 0] },
          );
        (le.box(c, 0.02, 0.31, 0.02, se - (O - 0.13) / 2, ve, ae + 0.022, {
          r: 0.003,
        }),
          le.box(c, 0.02, 0.31, 0.02, se + (O - 0.13) / 2, ve, ae + 0.022, {
            r: 0.003,
          }));
      }
      let k = I ? R + 0.07 : R + O - 0.07;
      (le.box(u, 0.045, 0.2, 0.02, k, U - 0.02, ae + 0.026, { r: 0.006 }),
        le.box(h, 0.028, 0.13, 0.018, k, U + 0, ae + 0.042, { r: 0.006 }),
        le.cyl(h, 0.012, 0.012, 0.012, k, U - 0.09, ae + 0.04, {
          seg: 8,
          rot: [es / 2, 0, 0],
        }));
      let ce = I ? R + O : R;
      for (let ve of [U - 0.5, U, U + 0.5])
        le.cyl(h, 0.011, 0.011, 0.09, ce, ve, ae + 0.016, { seg: 6 });
    };
  (o.push(_o(0, 0, M - 0.004)),
    T(o, 0.013, !0),
    o.decal(
      Mi(i, "RELAY " + s, { color: "yellow", font: "stencil", distress: 0.15 }),
      0.12,
      0.013 + O / 2,
      U + 0.02,
      0.018,
      null,
      O - 0.1,
    ),
    o.decal(
      Mi(i, "HARBORMASTER UPLINK", {
        color: "white",
        font: "sans",
        distress: 0.15,
      }),
      0.034,
      0.013 + O / 2,
      U - 0.1,
      0.018,
      null,
      O - 0.1,
    ),
    o.decal(
      Jn(i, "hazard_voltage"),
      0.13,
      0.013 + O / 2 + 0.1,
      U + 0.14,
      0.018,
      null,
    ),
    o.pop());
  let D = [-0.487, 0, M - 0.004];
  (T(r, 0, !1),
    r.decal(Jn(i, "hazard_general"), 0.13, O / 2, U + 0.1, 0.018, null),
    r.box(c, O - 0.1, 0.03, 0.014, O / 2, U + 0.55, -0.018, { r: 0.003 }),
    r.box(c, O - 0.1, 0.03, 0.014, O / 2, U - 0.55, -0.018, { r: 0.003 }),
    r.box(c, 0.03, 1.1, 0.014, 0.07, U, -0.018, { r: 0.003 }),
    r.box(c, 0.03, 1.1, 0.014, O - 0.07, U, -0.018, { r: 0.003 }),
    r.box(u, 0.24, 0.3, 0.02, O / 2, U - 0.05, -0.02, { r: 0.004 }),
    r.add(
      Mi(i, "WIRING DIAGRAM  SLT-R4", {
        color: "white",
        font: "sans",
        distress: 0.1,
      }).material,
      je(cn(0.2, 0.03), O / 2, U + 0.03, -0.0305, [0, es, 0]),
      null,
      !0,
    ));
  for (let le of [U + 0.36, U - 0.4])
    r.add(c, je(cn(O - 0.14, 0.3), O / 2, le, -0.0125, [0, es, 0]), g);
  r.decal(
    Mi(i, "ISOLATE BEFORE SERVICE", {
      color: "white",
      font: "sans",
      distress: 0.15,
    }),
    0.03,
    O / 2,
    U - 0.03,
    0.018,
    null,
    O - 0.1,
  );
  for (let le of [-1, 1]) {
    o.box(f, 0.012, 0.98, 0.5, le * 0.506, 1.06, 0, { r: 0.003 });
    for (let R = -6; R <= 6; R++)
      o.add(f, je($e(0.06, 0.94, 0.007), le * 0.54, 1.06, R * 0.038));
    o.box(c, 0.075, 0.03, 0.52, le * 0.54, 1.56, 0, { r: 0.006 });
  }
  for (let le = 0; le < 3; le++) {
    let R = -0.17 + le * 0.17,
      I = 0.44 - le * 0.02,
      se = 0.86 + le * 0.07;
    (o.tube(
      p,
      [
        [0.5, I, R],
        [0.6, I, R],
        [0.72 + le * 0.04, I - 0.06, R],
        [se - 0.02, 0.16, R + 0.02],
        [se, -0.03, R + 0.02],
      ],
      0.04,
      { seg: 8, tension: 0.4 },
    ),
      o.cyl(h, 0.055, 0.055, 0.05, 0.515, I, R, {
        seg: 8,
        flat: !0,
        rot: [0, 0, es / 2],
      }),
      o.cyl(h, 0.06, 0.066, 0.03, se, 0.015, R + 0.02, { seg: 8 }));
  }
  (o.tube(
    p,
    [
      [-0.3, 0.35, _],
      [-0.3, 0.33, _ - 0.1],
      [-0.34, 0.2, _ - 0.2],
      [-0.5, 0.165, _ - 0.3],
      [-0.9, 0.02, _ - 0.36],
      [-1.5, 0.018, _ - 0.2],
    ],
    0.022,
    { seg: 6 },
  ),
    o.box(u, 0.3, 0.22, 0.1, 0.2, 0.75, _ - 0.05, { r: 0.012 }),
    o.tube(
      p,
      [
        [0.2, 0.86, _ - 0.06],
        [0.2, 1.2, _ - 0.04],
        [-0.1, 1.55, _ - 0.03],
        [-0.3, 1.66, -0.16],
      ],
      0.012,
      { seg: 5 },
    ));
  let C = -0.3,
    G = -0.14,
    j = A + 0.05,
    N = 2.5,
    Q = [0, 1, 2].map((le) => [
      C + Math.cos(le * 2.094 + 0.52) * 0.085,
      G + Math.sin(le * 2.094 + 0.52) * 0.085,
    ]);
  o.box(h, 0.24, 0.012, 0.24, C, j + 0.006, G, { r: 0 });
  for (let le = 0; le < 3; le++) {
    let R = Q[le],
      I = Q[(le + 1) % 3];
    o.bar(h, [R[0], j, R[1]], [R[0], N, R[1]], 0.011, { seg: 6 });
    let se = 5,
      ae = (N - j) / se;
    for (let k = 0; k < se; k++)
      (o.bar(
        h,
        [R[0], j + k * ae, R[1]],
        [I[0], j + (k + 1) * ae, I[1]],
        0.005,
        { seg: 4 },
      ),
        o.bar(
          h,
          [R[0], j + (k + 1) * ae, R[1]],
          [I[0], j + (k + 1) * ae, I[1]],
          0.005,
          { seg: 4 },
        ));
  }
  (o.cyl(h, 0.11, 0.11, 0.014, C, N + 0.007, G, { seg: 10 }),
    o.cyl(u, 0.05, 0.06, 0.1, C, N + 0.06, G, { seg: 10 }));
  let K = (le, R, I) => {
    (o.cyl(u, 0.016, 0.02, 0.07, le, A + 0.085, R, { seg: 8 }),
      o.cyl(h, 0.011, 0.011, 0.06, le, A + 0.15, R, { seg: 6 }),
      o.bar(u, [le, A + 0.17, R], [le + 0.02, I, R - 0.015], 0.005, {
        r2: 0.0018,
        seg: 5,
        caps: !0,
      }));
  };
  (K(0.4, -0.24, 3.15),
    K(0.26, -0.24, 2.72),
    o.cyl(u, 0.03, 0.03, 0.26, 0.42, A + 0.18, 0.18, { seg: 8 }),
    o.add(u, je(mr(0.03, 8, 4, es * 2, 0, es / 2), 0.42, A + 0.31, 0.18)));
  let ie = g1(
    [
      [0, 0],
      [0.06, 0.006],
      [0.12, 0.024],
      [0.18, 0.054],
      [0.215, 0.078],
      [0.222, 0.078],
      [0.185, 0.05],
      [0.12, 0.016],
      [0.06, -0.003],
      [0, -0.01],
    ],
    18,
  );
  (ie.computeVertexNormals(),
    l.add(f, je(ie, 0, 0.3, 0.02, [es / 2 - 0.12, 0, 0])),
    l.cyl(f, 0.022, 0.03, 0.2, 0, 0.1, 0, { seg: 8 }),
    l.box(f, 0.07, 0.1, 0.06, 0, 0.26, -0.02, { r: 0.01 }));
  for (let le of [0.5, 2.6, 4.7])
    l.bar(
      f,
      [Math.cos(le) * 0.2, 0.3 + Math.sin(le) * 0.2, 0.09],
      [0, 0.325, 0.25],
      0.0045,
      { seg: 4 },
    );
  l.cyl(f, 0.022, 0.016, 0.05, 0, 0.325, 0.25, { seg: 8, rot: [es / 2, 0, 0] });
  let F = [C, N + 0.11, G];
  (o.box(u, 0.9, 0.07, 0.03, 0, A - 0.06, M + 0.012, { r: 0.008 }),
    a.add(x, je(cn(0.84, 0.036), 0, A - 0.06, M + 0.0275)));
  for (let le = 1; le < 6; le++)
    o.add(
      u,
      je($e(0.008, 0.05, 0.006), -0.42 + le * 0.14, A - 0.06, M + 0.028),
    );
  (o.box(u, 0.6, 0.06, 0.03, 0, A - 0.06, _ - 0.012, { r: 0.008 }),
    a.add(x, je(cn(0.54, 0.03), 0, A - 0.06, _ - 0.0275, [0, es, 0])));
  for (let le of [-1, 1])
    (o.box(u, 0.03, 1.2, 0.03, le * 0.5, 0.86, M, {
      r: 0.008,
      rot: [0, es / 4, 0],
    }),
      a.add(
        x,
        je(cn(0.014, 1.14), le * 0.5, 0.86, M + 0.0215, [
          0,
          (le * es) / 4,
          0,
        ]).translate(le * 0.0152, 0, 0),
      ),
      a.add(
        x,
        je(cn(0.014, 0.9), le * 0.5715, 1.06, 0.262, [0, (le * es) / 2, 0]),
      ));
  for (let le of [-1, 1])
    (o.box(u, 0.03, 0.06, 0.46, le * 0.587, A - 0.01, 0, { r: 0.008 }),
      a.add(
        x,
        je(cn(0.4, 0.032), le * 0.6025, A - 0.01, 0, [0, (le * es) / 2, 0]),
      ));
  (o.cyl(u, 0.028, 0.028, 0.02, C + 0.2, A + 0.06, G, { seg: 8 }),
    o.bar(h, [C + 0.2, A + 0.06, G], [C + 0.2, 2.64, G], 0.009, { seg: 6 }),
    a.add(x, je(ot(0.045, 0.045, 0.1, 10), C + 0.2, 2.69, G)),
    o.cyl(u, 0.05, 0.05, 0.014, C + 0.2, 2.747, G, { seg: 10 }),
    o.cyl(u, 0.05, 0.05, 0.014, C + 0.2, 2.633, G, { seg: 10 }),
    o.decal(Jn(i, "rust_run"), 0.5, 0.5135, 0.3, 0.2, [0, es / 2, 0], 0.14),
    o.decal(Jn(i, "grime_streak"), 0.6, -0.2, 1.2, _ - 0.001, [0, es, 0], 0.2));
  let X = o.group(),
    ne = new wt();
  ((ne.userData.noMerge = !0), (ne.name = "relay_dynamic"), X.add(ne));
  let re = i.mats.emissive(16752680, 4);
  re = re.clone();
  let $ = new Qe(Ii(a.buckets.get(x).geos), re);
  (($.userData.noMerge = !0), ne.add($));
  let ue = r.group().children.slice();
  for (let le of ue)
    (le.position.set(D[0], D[1], D[2]), (le.userData.noMerge = !0), ne.add(le));
  let me = l.group().children.slice();
  for (let le of me)
    (le.position.set(F[0], F[1], F[2]), (le.userData.noMerge = !0), ne.add(le));
  let ee = new Ve(),
    B = (le, R) => {
      if (re.userData && typeof re.userData.setEmissive == "function") {
        re.userData.setEmissive(le, R);
        return;
      }
      re.emissive && re.emissive.isColor && re.isMeshStandardMaterial
        ? (re.emissive.copy(le), (re.emissiveIntensity = R))
        : re.uniforms &&
            re.uniforms.color &&
            re.uniforms.color.value &&
            re.uniforms.color.value.isColor
          ? (re.uniforms.color.value.copy(le).multiplyScalar(R),
            re.uniforms.intensity && (re.uniforms.intensity.value = 1))
          : re.color && re.color.isColor && re.color.copy(le).multiplyScalar(R);
    },
    P = "live",
    V = (t % 7) * 0.9,
    Y = 0,
    L = 0.7,
    Z = 0,
    J = 0,
    oe = 0,
    fe = X.userData;
  if (
    ((fe.state = P),
    (fe.beacon = [C + 0.2, 2.68, G]),
    (fe.interact = [0, 1, 0.75]),
    (fe.emissiveMaterial = re),
    (fe.level = 0),
    (fe.setState = (le) => {
      (le !== "live" && le !== "cutting" && le !== "cut") ||
        (le !== P &&
          ((P = le),
          (fe.state = le),
          le === "cut" && ((Y = 0), (oe = 1.32), (J = 2.5)),
          le === "live" && (oe = 0)));
    }),
    (fe.update = (le) => {
      ((le = Math.min(le || 0, 0.1)), (V += le));
      let R = 0,
        I = Kg,
        se = 0.7;
      if (P === "live") R = 5.5 + 3 * Math.sin(V * 2.2);
      else if (P === "cutting") {
        let ae = Math.sin(V * 2 * es * 5.5) > 0;
        ((R = ae ? 9 : 0.5),
          (I = ae ? kA : Kg),
          (se = 2.6 + Math.sin(V * 9) * 1.5));
      } else if (((Y += le), (I = LA), (se = 0), Y < 0.3)) R = 9;
      else if (Y < 2.6) {
        let ae = 1 - (Y - 0.3) / 2.3;
        R =
          7 *
          ae *
          ae *
          (Math.sin(Y * 37) * Math.sin(Y * 13.3) > -0.2 ? 1 : 0.08);
      } else R = 0;
      if (
        ((fe.level = R),
        B(ee.copy(I), R),
        (L += (se - L) * Math.min(1, le * (P === "cut" ? 1.6 : 3))),
        Math.abs(L) > 0.001)
      )
        for (let ae of me) ae.rotateY(L * le);
      if (Math.abs(oe - Z) > 1e-4 || Math.abs(J) > 0.001) {
        J += (-(Z - oe) * 30 - J * 3.2) * le;
        let ae = Z + J * le;
        (ae < 0 && ((ae = 0), (J = Math.abs(J) * 0.3)),
          ae > 1.9 && ((ae = 1.9), (J = -Math.abs(J) * 0.4)));
        let k = ae - Z;
        Z = ae;
        for (let ce of ue) ce.rotateY(-k);
      }
    }),
    e.state === "cut")
  ) {
    fe.setState("cut");
    for (let le = 0; le < 80; le++) fe.update(0.05);
  } else
    (e.state === "cutting" && fe.setState("cutting"),
      fe.update(0),
      P === "live" && B(ee.copy(Kg), 6));
  return qi(X, "relay_node", Gi.relay_node.slice(0, 3), [
    { min: [-0.68, 0, -0.48], max: [0.68, 0.16, 0.48], surface: "concrete" },
    { min: [-0.57, 0.16, -0.32], max: [0.57, 1.67, 0.33], surface: "metal" },
    {
      min: [-0.42, 1.67, -0.26],
      max: [-0.18, 2.5, -0.02],
      surface: "metal",
      bulletOnly: !0,
    },
  ]);
}
