from engine import Triangle, Camera
from utils import Player
from model import cube, square
import numpy as np

def main():
    ts, c, f = cube()
    p = Player()
    p.play_f(f)


if __name__ == '__main__':
    main()
