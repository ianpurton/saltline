#!/usr/bin/env python3
"""Deterministic original procedural sound palette; no external audio assets."""
from pathlib import Path
import math, random, struct, wave
out=Path(__file__).resolve().parent.parent/'assets/audio'
out.mkdir(parents=True,exist_ok=True)
rate=24000
for name,duration in [('shot',.32),('impact',.2),('relay',.65),('reload',1.8),('ui',.15),('rain',8),('machinery',8),('enemy',.18)]:
    rng=random.Random(1337);samples=[];low=0
    for i in range(int(rate*duration)):
        t=i/rate;noise=rng.uniform(-1,1);low=low*.94+noise*.06
        if name=='shot': v=(noise*.7+math.sin(2*math.pi*(105*t-60*t*t))*.7)*math.exp(-t*24)
        elif name=='impact':v=(noise*.7+math.sin(t*4100)*.25)*math.exp(-t*35)
        elif name=='relay':v=(noise*.4+math.sin(t*(1800-900*t))*.4)*math.exp(-t*6)
        elif name=='reload':
            v=sum((noise*.5+math.sin(t*3300)*.2)*math.exp(-(t-k)*65) for k in [.1,.55,1.25,1.6] if t>=k)
        elif name=='ui':v=math.sin(t*2*math.pi*880)*math.exp(-t*35)*.3
        elif name=='enemy':v=math.sin(t*2*math.pi*(680+120*math.sin(t*18)))*math.exp(-t*22)*.3
        elif name=='rain':v=noise*.13+low*.8
        else:v=.13*math.sin(t*2*math.pi*60)+.05*math.sin(t*2*math.pi*121)+low*.35
        if name in ('rain','machinery'):v*=min(1,t/.05,(duration-t)/.05)
        else:v*=min(1,t/.001,(duration-t)/.005)
        samples.append(struct.pack('<h',int(max(-1,min(1,v))*.85*32767)))
    with wave.open(str(out/(name+'.wav')),'wb') as f:
        f.setnchannels(1);f.setsampwidth(2);f.setframerate(rate);f.writeframes(b''.join(samples))
