import numpy as np
from engine import Triangle, Camera, DirectLighting, rotate_point_around_axis

def cube():
    R3 = np.sqrt(3)
    R2 = np.sqrt(2)
    R12 = np.sqrt(12)
    r = R2/R3

    A = np.array([0, 0, R3/2])
    B = np.array([0, 0, -R3/2])
    C = np.array([r, 0, 1/R12])
    D = np.array([-r/2, r*R3/2, 1/R12])
    E = np.array([-r/2, -r*R3/2, 1/R12])
    F = np.array([r/2, r*R3/2, -1/R12])
    G = np.array([-r, 0, -1/R12])
    H = np.array([r/2, -r*R3/2, -1/R12])
    for i in [A, B, C, D, E, F, G, H]:
        assert(np.isclose(np.linalg.norm(i), R3/2))
    return [
        Triangle(A, D, C, '-'),
        Triangle(C, D, F, '-'),
        Triangle(A, E, D, '*'),
        Triangle(D, E, G, '*'),
        Triangle(A, C, E, '.'),
        Triangle(E, C, H, '.'),
        Triangle(D, G, F, '#'),
        Triangle(F, G, B, '#'),
        Triangle(C, F, H, '/'),
        Triangle(H, F, B, '/'),
        Triangle(E, H, G, '@'),
        Triangle(G, H, B, '@')
    ]

def square():
    R3 = np.sqrt(3)
    A = np.array([0, -R3/2, R3/2])
    B = np.array([0, R3/2, R3/2])
    C = np.array([0, R3/2, -R3/2])
    D = np.array([0, -R3/2, -R3/2])

    return [
        Triangle(A, B, C, '@'),
        Triangle(A, C, D, '.')
    ]
