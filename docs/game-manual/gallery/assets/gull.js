// Extracted original builder(s): j_. Shared dependencies are in shared/reference.js.
function j_() {
  let i = xm,
    e = new iu(),
    t = new iu(),
    n = (l, c, u, h, f) => {
      (e.bone(l, c, u, h, f), t.bone(l, c, u, h, f));
    };
  (n("root", null, 0, 0, 0), n("body", "root", 0, 0, 0));
  for (let l = 0; l < 4; l++)
    n("rotor" + l, "body", i.rotors[l][0], i.rotors[l][1], i.rotors[l][2]);
  (n("gimbalYaw", "body", i.gimbal[0], i.gimbal[1], i.gimbal[2]),
    n("ball", "gimbalYaw", i.ball[0], i.ball[1], i.ball[2]),
    n("lamp", "ball", i.lamp[0], i.lamp[1], i.lamp[2]),
    n("gun", "body", i.gun[0], i.gun[1], i.gun[2]),
    n("strobeG", "body", 0, 0, 0),
    n("strobeR", "body", 0, 0, 0),
    n("strobeW", "body", 0, 0, 0));
  let s = (l, c, u, h, f) => e.add(l, c, u, h, f);
  (s("shell", "body", Ce(Ur(1, 20, 12, 0.118, 0.074, 0.27), 0, 0.02, 0), Ss),
    s(
      "shell",
      "body",
      Ce(ot(1, 0.9, 1, 20), 0, 0.02, 0.165, va, 0, 0, [0.094, 0.05, 0.06]),
      Ts,
    ),
    s(
      "shell",
      "body",
      Ce(ot(1, 1, 1, 20), 0, 0.02, -0.02, va, 0, 0, [0.121, 0.012, 0.0765]),
      s0,
    ),
    s("shell", "body", Ce(ft(0.118, 0.014, 0.2, 0.005), 0, 0.089, -0.03), vm),
    s("shell", "body", Ce(ft(0.03, 0.006, 0.2, 0.002), 0, 0.0975, -0.03), Ts));
  for (let l of [-0.05, 0.05])
    for (let c of [-0.12, 0.06]) s("metal", "body", Yv(l, 0.097, c), Yn);
  (s("metal", "body", Ce(ft(0.175, 0.06, 0.37, 0.016), 0, -0.035, -0.005), an),
    s("metal", "body", Ce(ft(0.12, 0.075, 0.11, 0.014), 0, 0.005, -0.265), As),
    s("shell", "body", Ce(ft(0.124, 0.02, 0.03, 0.005), 0, 0.03, -0.3), Ts),
    s(
      "glow",
      "body",
      Ce(ft(0.05, 0.006, 0.003, 0.001), 0, 0, -0.321),
      [0.3, 1.6, 0.4],
      !1,
    ));
  for (let l of [1, -1]) {
    for (let c = 0; c < 4; c++)
      s(
        "metal",
        "body",
        Ce(ft(0.004, 0.01, 0.06, 0.001), l * 0.0885, -0.045 + c * 0.016, -0.06),
        [0.2, 0.2, 0.2],
      );
    s(
      "metal",
      "body",
      Fn(
        [
          [l * 0.075, -0.055, 0.13],
          [l * 0.135, -0.175, 0.16],
          [l * 0.135, -0.192, 0.11],
          [l * 0.135, -0.192, -0.14],
          [l * 0.135, -0.175, -0.19],
          [l * 0.075, -0.055, -0.15],
        ],
        0.0085,
        6,
        !1,
        0.5,
      ),
      Yn,
    );
    for (let c of [0.07, -0.1])
      s(
        "shell",
        "body",
        Ce(Si(0.0135, 0.0135, 0.07, 8), l * 0.135, -0.192, c),
        ga,
      );
    (s(
      "metal",
      "body",
      su([l * 0.04, 0.07, -0.17], [l * 0.085, 0.37, -0.27], 0.004, 0.002, 5),
      an,
    ),
      s(
        "metal",
        "body",
        Ce(ds(0.008, 0.01, 0.025, 6), l * 0.04, 0.078, -0.17),
        Yn,
      ));
  }
  (s("metal", "body", Ce(ds(0.007, 0.007, 0.04, 6), 0, 0.105, 0.1), Yn),
    s("shell", "body", Ce(ds(0.034, 0.036, 0.018, 14), 0, 0.132, 0.1), Ss),
    s("metal", "body", Ce(ft(0.07, 0.03, 0.09, 0.008), 0, -0.075, 0.235), As));
  let o = FC();
  for (let l = 0; l < 4; l++) {
    let [c, u, h] = i.rotors[l],
      f = Math.sign(c),
      p = Math.sign(h);
    (s(
      "metal",
      "body",
      su([f * 0.07, 0, p * 0.09], [c, u - 0.05, h], 0.017, 0.015, 8),
      an,
    ),
      s(
        "shell",
        "body",
        Ce(
          ft(0.07, 0.045, 0.07, 0.014),
          f * 0.085,
          0,
          p * 0.1,
          0,
          f * p * 0.74,
          0,
        ),
        Ss,
      ),
      s(
        "shell",
        "body",
        Fn(
          [
            [f * 0.09, 0.02, p * 0.12],
            [f * 0.18, 0.012, p * 0.19],
            [c - f * 0.05, u - 0.03, h - p * 0.04],
          ],
          0.005,
          5,
        ),
        ga,
      ),
      s("metal", "body", Ce(ds(0.04, 0.03, 0.075, 12), c, u - 0.05, h), an),
      s("metal", "body", Ce(ds(0.042, 0.042, 0.012, 12), c, u - 0.02, h), Yn),
      s("shell", "body", Ce(o.clone(), c, u, h), Ss),
      s(
        "shell",
        "body",
        Ce(new Ci(0.196, 0.0085, 6, 28), c, u + 0.036, h, va, 0, 0),
        Ts,
      ));
    for (let v = 0; v < 3; v++) {
      let g = v * 2.094 + 0.5 + l;
      s(
        "metal",
        "body",
        Ce(
          ft(0.145, 0.007, 0.016, 0.002),
          c + Math.cos(g) * 0.105,
          u - 0.022,
          h - Math.sin(g) * 0.105,
          0,
          g,
          0,
        ),
        an,
      );
    }
    for (let v = -1; v <= 1; v++) {
      let g = Math.atan2(p, f) + v * 0.2;
      s(
        "shell",
        "body",
        Ce(
          ft(0.012, 0.05, 0.036, 0.003),
          c + Math.cos(g) * 0.205,
          u,
          h + Math.sin(g) * 0.205,
          0,
          -g,
          0,
        ),
        v === 0 ? s0 : Of,
      );
    }
    (s("metal", "rotor" + l, Ce(ds(0.017, 0.02, 0.03, 8), 0, -0.002, 0), Yn),
      s(
        "metal",
        "rotor" + l,
        Ce(ft(0.158, 0.0035, 0.03, 0.001), 0.088, 0.008, 0, 0.22, 0, 0),
        an,
      ),
      s(
        "metal",
        "rotor" + l,
        Ce(ft(0.158, 0.0035, 0.03, 0.001), -0.088, 0.008, 0, -0.22, 0, 0),
        an,
      ),
      s(
        "shell",
        "rotor" + l,
        Ce(ft(0.02, 0.004, 0.031, 0.001), 0.158, 0.008, 0, 0.22, 0, 0),
        Of,
      ),
      s(
        "shell",
        "rotor" + l,
        Ce(ft(0.02, 0.004, 0.031, 0.001), -0.158, 0.008, 0, -0.22, 0, 0),
        Of,
      ));
    let d = new O0(0.03, 0.168, 24, 1);
    (d.rotateX(-va),
      t.add("disc", "body", Ce(d, c, u + 0.008, h), [1, 1, 1], !1));
  }
  (s(
    "glow",
    "strobeR",
    Ce(Ur(0.012, 8, 6), 0.33 + 0.215, 0.035, 0.3),
    [10, 0.4, 0.2],
    !1,
  ),
    s(
      "glow",
      "strobeR",
      Ce(Ur(0.012, 8, 6), 0.33 + 0.215, 0.035, -0.3),
      [10, 0.4, 0.2],
      !1,
    ),
    s(
      "glow",
      "strobeG",
      Ce(Ur(0.012, 8, 6), -0.33 - 0.215, 0.035, 0.3),
      [0.3, 9, 1],
      !1,
    ),
    s(
      "glow",
      "strobeG",
      Ce(Ur(0.012, 8, 6), -0.33 - 0.215, 0.035, -0.3),
      [0.3, 9, 1],
      !1,
    ),
    s("glow", "strobeW", Ce(Ur(0.011, 8, 6), 0, 0.05, -0.325), [9, 9, 9], !1),
    s("metal", "gimbalYaw", Ce(ds(0.036, 0.036, 0.014, 12), 0, -0.004, 0), an),
    s(
      "metal",
      "gimbalYaw",
      Ce(ft(0.156, 0.01, 0.03, 0.003), 0, -0.014, 0),
      As,
    ));
  for (let l of [1, -1])
    s(
      "metal",
      "gimbalYaw",
      Ce(ft(0.009, 0.075, 0.03, 0.003), l * 0.0735, -0.05, 0),
      As,
    );
  (s("metal", "ball", Ce(Ur(0.06, 18, 12), 0, 0, 0), an),
    s("metal", "ball", Ce(Ro(0.013, 0.013, 0.142, 8), 0, 0, 0), Yn),
    s("metal", "ball", Ce(Si(0.031, 0.036, 0.03, 14), 0, 0.004, 0.048), As),
    s(
      "lens",
      "ball",
      Ce(Si(0.025, 0.025, 0.004, 14), 0, 0.004, 0.0645),
      [1, 1, 1],
      !1,
    ),
    s(
      "metal",
      "ball",
      Ce(Si(0.012, 0.012, 0.01, 8), 0.03, 0.036, 0.04),
      [0.1, 0.12, 0.2],
    ),
    s(
      "metal",
      "ball",
      Ce(Si(0.019, 0.023, 0.045, 10), i.lamp[0], i.lamp[1], i.lamp[2] - 0.022),
      an,
    ),
    s(
      "glow",
      "lamp",
      Ce(Si(0.016, 0.016, 0.003, 10), 0, 0, 0.001),
      [11, 9.8, 7.6],
      !1,
    ));
  {
    let u = new Ua(1.25, 9, 20, 6, !0);
    (u.translate(0, -9 / 2, 0),
      u.rotateX(-va),
      t.add("cone", "lamp", u, [1, 1, 1], !1));
  }
  (s("metal", "gun", Ce(ft(0.05, 0.03, 0.05, 0.006), 0, 0.012, 0), As),
    s("metal", "gun", Ce(Ro(0.012, 0.012, 0.07, 8), 0, -0.005, 0), Yn),
    s("metal", "gun", Ce(ft(0.052, 0.046, 0.13, 0.008), 0, -0.032, 0), As),
    s("shell", "gun", Ce(ft(0.054, 0.012, 0.05, 0.003), 0, -0.02, -0.03), Ts));
  for (let l of [-0.014, 0.014])
    (s("metal", "gun", Ce(Si(0.0085, 0.0085, 0.2, 8), l, -0.032, 0.16), an),
      s("metal", "gun", Ce(Si(0.012, 0.012, 0.03, 8), l, -0.032, 0.255), As));
  (s("metal", "gun", Ce(ft(0.045, 0.012, 0.012, 0.002), 0, -0.032, 0.2), Yn),
    s(
      "metal",
      "gun",
      Ce(ft(0.03, 0.05, 0.05, 0.006), 0.04, -0.035, -0.02),
      an,
    ));
  let r = e.build(["shell", "metal", "glow", "lens"]),
    a = t.build(["disc", "cone"]);
  return ((r.extra = a), r);
}
