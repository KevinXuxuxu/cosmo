import numpy as np

BRIGHT_TILES = [".", ",", ":", ";", "!", "~", "-", "+", "=", "*", "#", "%", "@", "M"]
BL = len(BRIGHT_TILES)

def unit(v):
    l = np.linalg.norm(v)
    return v / l
    

def rotate_point_around_axis(P, P0, d, theta):
    # Translate point P to the origin relative to P0
    P_rel = P - P0
    # Normalize the direction vector d
    d_norm = unit(d)
    # Compute the rotated point using Rodrigues' rotation formula
    cos_theta = np.cos(theta)
    sin_theta = np.sin(theta)
    dot_product = np.dot(d_norm, P_rel)
    cross_product = np.cross(d_norm, P_rel)
    
    P_rot = (P_rel * cos_theta +
             cross_product * sin_theta +
             d_norm * dot_product * (1 - cos_theta))
    # Translate the point back
    P_final = P_rot + P0
    return P_final

class Triangle:

    def __init__(self, A, B, C, color):
        # ABC should be in clock-wise, normal direction pointing up
        self.A = A
        self.B = B
        self.C = C
        self.color = color
        self._preprocess()

    def _preprocess(self):
        self.v0 = self.C - self.A
        self.v1 = self.B - self.A
        self.dot00 = np.dot(self.v0, self.v0)
        self.dot01 = np.dot(self.v0, self.v1)
        self.dot11 = np.dot(self.v1, self.v1)
        self.invDenom = 1 / (self.dot00 * self.dot11 - self.dot01 * self.dot01)
        self.n = unit(np.cross(self.v1, self.v0))

    def is_point_in(self, P):
        v2 = P - self.A
        dot02 = np.dot(self.v0, v2)
        dot12 = np.dot(self.v1, v2)
        u = (self.dot11 * dot02 - self.dot01 * dot12) * self.invDenom
        v = (self.dot00 * dot12 - self.dot01 * dot02) * self.invDenom
        return (u >= 0) and (v >= 0) and (u + v < 1)

    def intersect(self, P0, d):
        denom = np.dot(self.n, d)
        if denom > -1e-6:
            return False  # Line is parallel to plane or it's blocked 
        t = np.dot(self.n, self.A - P0) / denom
        P = P0 + t * d
        return self.is_point_in(P)

    def translate(self, d):
        self.A += d
        self.B += d
        self.C += d
        self._preprocess()

    def rotate(self, P0, d, theta):
        self.A = rotate_point_around_axis(self.A, P0, d, theta)
        self.B = rotate_point_around_axis(self.B, P0, d, theta)
        self.C = rotate_point_around_axis(self.C, P0, d, theta)
        self._preprocess()


class DirectLighting:

    def __init__(self, d):
        self.d = unit(d)

    def light(self, n):
        if np.dot(self.d, n) > -1e-6:
            return 0
        # assume all unit vectors
        return -np.dot(self.d, n)

    def type(self):
        return "Direct"


class Camera:

    def __init__(self, f, d, x, y, n):
        self.f = f
        self.d = d
        self.y = y
        self.x = x
        self.P0 = np.array([d, 0, 0])
        self.n = n
        self.m = int(n / y * x * 2)
        self.dy = y / n
        self.dx = self.dy / 2

    def _d(self, i, j):
        pix = np.array([
            self.d - self.f,
            -self.x/2 + j * self.dx,
            -self.y/2 + i * self.dy
        ])
        return unit(pix - self.P0)

    def look(self, ts, lighting = None):
        r = [[' ']*self.m for _ in range(self.n)]
        if lighting and lighting.type() == "Direct":
            for t in ts:
                brightness = lighting.light(t.n)
                t.color = BRIGHT_TILES[int(brightness*BL)]
        for i in range(self.n):
            for j in range(self.m):
                for t in ts: # check all triangles
                    if t.intersect(self.P0, self._d(i, j)):
                        r[i][j] = t.color
                        break
                    else:
                        r[i][j] = ' '
        return r

