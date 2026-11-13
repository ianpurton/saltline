// Extracted original builder(s): QA. Shared dependencies are in shared/reference.js.
function QA(i, e) {
  let t = new Kn(i, "tarp_cargo", e.seed || 0),
    n = t.rng,
    s = t.seed * 1.7,
    o = 0.144,
    r = [
      [-1.06, -0.12, -0.58, 0.58, n.range(0.98, 1.12)],
      [-0.06, 1.06, -0.58, 0.12, n.range(1.32, 1.42)],
      [0.02, 0.62, 0.16, 0.58, n.range(0.72, 0.86)],
      [0.66, 1.06, 0.18, 0.56, n.range(0.5, 0.62)],
    ];
  for (let N of [-0.6, 0.6])
    (t.push({ p: [N, 0, 0], r: [0, 0, 0] }), S1(t, n, { stamp: !1 }), t.pop());
  for (let N of r) {
    let Q = N[1] - N[0],
      K = N[3] - N[2],
      ie = N[4] - o,
      F = (N[0] + N[1]) / 2,
      X = (N[2] + N[3]) / 2,
      ne = n.range(0.7, 0.95);
    (t.add("wood_pallet", Ge(Q, ie, K, 0.006), { p: [F, o + ie / 2, X] }, [
      ne,
      ne * 0.95,
      ne * 0.88,
    ]),
      t.add(
        "wood_pallet",
        $e(Q + 0.012, 0.07, K + 0.012),
        { p: [F, o + 0.04, X] },
        ne * 0.75,
      ));
  }
  let a = 1.9,
    l = 22,
    c = 14,
    u = -1.125,
    h = 1.125,
    f = -0.645,
    p = 0.645,
    d = 5,
    v = (N, Q) => {
      let K = 0.3;
      for (let ie of r) {
        let F = Math.max(ie[0] - N, 0, N - ie[1]),
          X = Math.max(ie[2] - Q, 0, Q - ie[3]),
          ne = Math.hypot(F, X);
        K = Math.max(K, ie[4] - 0.95 * ne - 0.9 * ne * ne);
      }
      return (
        K +
        0.016 +
        0.022 * gn(N * 5 + s, Q * 5) +
        0.008 * Math.sin(N * 23 + Q * 7 + s) +
        0.006 * Math.sin(Q * 31 - N * 5)
      );
    },
    g = [],
    x = [],
    y = [],
    b = (N, Q) => Q * (l + 1) + N;
  for (let N = 0; N <= c; N++)
    for (let Q = 0; Q <= l; Q++) {
      let K = u + ((h - u) * Q) / l,
        ie = f + ((p - f) * N) / c;
      (g.push(K, v(K, ie), ie), x.push(K * a, ie * a));
    }
  for (let N = 0; N < c; N++)
    for (let Q = 0; Q < l; Q++) {
      let K = b(Q, N),
        ie = b(Q + 1, N),
        F = b(Q + 1, N + 1),
        X = b(Q, N + 1);
      y.push(K, X, ie, ie, X, F);
    }
  let m = [];
  for (let N = 0; N < l; N++) m.push([N, 0, 0, -1]);
  for (let N = 0; N < c; N++) m.push([l, N, 1, 0]);
  for (let N = l; N > 0; N--) m.push([N, c, 0, 1]);
  for (let N = c; N > 0; N--) m.push([0, N, -1, 0]);
  let A = m.length,
    _ = (l + 1) * (c + 1),
    M = new Map(),
    E = n() * A;
  m.forEach((N, Q) => {
    M.set(N[0] + "," + N[1], Q);
    let K = (N[0] === 0 || N[0] === l) && (N[1] === 0 || N[1] === c),
      ie = N[2],
      F = N[3];
    K &&
      ((ie = N[0] === 0 ? -0.707 : 0.707), (F = N[1] === 0 ? -0.707 : 0.707));
    let X = b(N[0], N[1]),
      ne = g[X * 3],
      re = g[X * 3 + 1],
      $ = g[X * 3 + 2],
      ue = Math.abs(Q - E);
    ue = Math.min(ue, A - ue);
    let me =
        0.17 +
        0.07 * Math.sin(Q * 0.55 + s) +
        0.07 * gn(Q * 0.31 + s, 2.2) +
        0.3 * Math.exp((-ue * ue) / 14),
      ee = n() * 6;
    for (let B = 1; B <= d; B++) {
      let P = B / d,
        V = re + (me - re) * (0.25 * P + 0.75 * P * P) - (B === 1 ? 0.02 : 0),
        Y =
          (N[0] === 5 || N[0] === 16) && N[2] === 0
            ? 0.035 * Math.sin(Math.PI * P)
            : 0,
        L =
          P *
            (0.04 * Math.sin(Q * 1.7 + ee * 0.2) +
              0.028 * Math.sin(Q * 0.83 + 1.3 + s)) +
          0.02 * P * gn(Q * 0.8, B * 0.9 + s) -
          Y,
        Z = (B === 1 ? 0.028 : 0.04 + 0.03 * P) + L + (K ? 0.012 * P : 0),
        J = zi(ne + ie * Z, -1.2, 1.2),
        oe = zi($ + F * Z, -0.7, 0.7),
        fe = re - V;
      (g.push(J, V, oe),
        x.push((ne + ie * (Z + fe)) * a, ($ + F * (Z + fe)) * a));
    }
  });
  let w = (N, Q) => (Q === 0 ? b(m[N][0], m[N][1]) : _ + N * d + (Q - 1));
  for (let N = 0; N < A; N++)
    for (let Q = 0; Q < d; Q++) {
      let K = (N + 1) % A,
        ie = w(N, Q),
        F = w(K, Q),
        X = w(K, Q + 1),
        ne = w(N, Q + 1);
      y.push(ie, F, ne, F, X, ne);
    }
  let S = new lt.BufferGeometry();
  (S.setAttribute("position", new lt.BufferAttribute(new Float32Array(g), 3)),
    S.setAttribute("uv", new lt.BufferAttribute(new Float32Array(x), 2)),
    S.setIndex(y),
    S.computeVertexNormals());
  let z = S.attributes.normal,
    H = S.attributes.position,
    O = (N, Q) => [
      H.getX(N) + z.getX(N) * Q,
      H.getY(N) + z.getY(N) * Q,
      H.getZ(N) + z.getZ(N) * Q,
    ],
    q = [];
  for (let N of [5, 16]) {
    let Q = [],
      K = M.get(N + ",0"),
      ie = M.get(N + "," + c);
    Q.push([H.getX(w(K, d)), 0.11, -0.52]);
    for (let F = d; F >= 1; F--) Q.push(O(w(K, F), 0.007));
    for (let F = 0; F <= c; F++) Q.push(O(b(N, F), 0.007));
    for (let F = 1; F <= d; F++) Q.push(O(w(ie, F), 0.007));
    (Q.push([H.getX(w(ie, d)), 0.11, 0.52]), q.push(Q));
  }
  let U = 6,
    T = [],
    D = M.get("0," + U),
    C = M.get(l + "," + U);
  T.push([-1.1, 0.1, H.getZ(w(D, d))]);
  for (let N = d; N >= 1; N -= 2) T.push(O(w(D, N), 0.012));
  for (let N = 0; N <= l; N += 2) T.push(O(b(N, U), 0.012));
  for (let N = 1; N <= d; N += 2) T.push(O(w(C, N), 0.012));
  (T.push([1.1, 0.1, H.getZ(w(C, d))]),
    t.add(t.mat("tarp", { doubleSide: !0 }), S, null, (N, Q, K) => {
      let ie = gn(N * 2.3 + s, Q * 2.3, K * 2.3),
        F = gn(N * 11, Q * 11, K * 11 + s);
      return (
        (0.72 + 0.28 * ie) * (0.9 + 0.1 * F) * (0.8 + 0.2 * Q3(0.15, 0.6, Q))
      );
    }));
  for (let N of q) {
    t.add("paint_orange", vr(N, [0.024, 0, 0]), null, [1, 0.82, 0.5]);
    let Q = N[N.length - 4];
    (t.add(
      "steel_worn",
      Ge(0.065, 0.1, 0.03, 0.006),
      { p: [Q[0], Q[1], Q[2] + 0.014], r: [0.25, 0, 0] },
      0.8,
    ),
      t.add(
        "steel_dark",
        Ge(0.02, 0.09, 0.012, 0.003),
        { p: [Q[0] + 0.03, Q[1] + 0.06, Q[2] + 0.04], r: [0.9, 0, 0] },
        0.8,
      ));
  }
  t.add("rope", Fn(T, 0.009, 4, !1, 0.5), null, 0.85);
  let G = [2.4, 1.5, 1.4],
    j = Math.max(...r.map((N) => N[4])) + 0.04;
  return t.finish({
    size: G,
    colliders: [{ min: [-1.2, 0, -0.7], max: [1.2, j, 0.7], surface: "dirt" }],
  });
}
