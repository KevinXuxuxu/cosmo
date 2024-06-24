import models
import numpy as np
from engine import Camera, DirectLighting

O = np.array([0, 0 ,0])
deg = np.pi/180
R3 = np.sqrt(3)

def cube(lighting = False):
    ts = models.cube()
    c = Camera(R3/2, 10*R3, R3/14, R3/14, 40)
    l = DirectLighting(np.array([-2, 0, -1]))
    def f(t):
        axis = np.array([0, np.sin(t), np.cos(t)])
        for t in ts:
            t.rotate(O, axis, 15*deg)
        return [''.join(row) for row in c.look(ts, l if lighting else None)]
    return ts, c, f

def square():
    ts = models.square()
    c = Camera(R3/2, 1.5*R3, R3/2, R3/2, 20)
    def f(t):
        for t in ts:
            t.rotate(O, np.array([0, 0, 1]), 5*deg)
        return [''.join(row) for row in c.look(ts)]
    return ts, c, f