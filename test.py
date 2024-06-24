from engine import Triangle, Camera
from utils import Player
import numpy as np

O = np.array([0, 0 ,0])
R3 = np.sqrt(3)
R15 = np.sqrt(15)
r = R3/3
deg = np.pi/180

def main():
    A = np.array([0, 0, R3/2])
    B = np.array([0, 0, -R3/2])
    C = np.array([r, 0, R15/6])
    D = np.array([-r/2, r*R3/2, R15/6])
    E = np.array([-r/2, -r*R3/2, R15/6])
    F = np.array([r/2, r*R3/2, -R15/6])
    G = np.array([-r, 0, -R15/6])
    H = np.array([r/2, -r*R3/2, -R15/6])
    # for i in [A, B, C, D, E, F, G, H]:
    #     assert(np.isclose(np.linalg.norm(i), R3/2))
    ts = [
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

    c = Camera(R3/2, 1.5*R3, R3/2, R3/2, 40)
    def f(t):
        for t in ts:
            t.rotate(O, np.array([0, 0, 1]), 30*deg)
        return [''.join(row) for row in c.look(ts)]
    
    p = Player()
    p.play_f(f)


if __name__ == '__main__':
    main()
