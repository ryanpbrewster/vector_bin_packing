import matplotlib.pyplot as plt
import sys

data = [ float(line) for line in sys.stdin.readlines() ]
plt.hist(data, bins = 50)
plt.show()
