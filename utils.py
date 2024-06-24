import sys
import time
import math

class Player:

    def __init__(self, fr = 24):
        self.fr = fr
        self.wait_time = 1 / self.fr

    def _render(self, lines):
        # Move the cursor up by the number of lines in the frame
        sys.stdout.write("\033[F" * len(lines))
        for line in lines:
            sys.stdout.write("\033[K" + line + "\n")  # Clear the line and print new content
        sys.stdout.flush()

    def play_f(self, f, duration = 60):
        t = 0
        while t < duration:
            self._render(f(t))
            time.sleep(self.wait_time)
            t += self.wait_time

def sin(size, t):
    x, y = size
    r = [[' ']*x for _ in range(y)]
    for i in range(y):
        j = min(x-1, int((math.sin(4*t+i/y*2*math.pi)+1)*x/2))
        r[i][j] = '*'
    return [''.join(i) for i in r]

def main():
    p = Player((20, 20))
    p.play_f(sin)

if __name__ == '__main__':
    sys.exit(main())
