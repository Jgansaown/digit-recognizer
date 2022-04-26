import csv
import matplotlib.pyplot as plt
import numpy as np
from scipy.signal import savgol_filter


with open('lc-output.txt', 'r') as f:
    reader = csv.reader(f, delimiter=',')
    
    lc_data = {}
    for row in reader:
        total, rate, *data = row
        if total == 'total': #row is header
            lc_data['header'] = {
                # 'total': 0,
                'rates': [],
                'indexes': [int(d) for d in data]
            }
        else:
            # lc_data['header']['total'] = int(total)
            lc_data['header']['rates'].append(float(rate))
            lc_data[float(rate)] = [int(d)/int(total) for d in data]

# print(X, Y)
fig, ax = plt.subplots()


for rate in lc_data['header']['rates']:
    # if rate in [0.1, 0.01, 0.001]:
        yhat = savgol_filter(lc_data[rate], 51, 3) # window size 51, polynomial order 3
        line = ax.plot(lc_data['header']['indexes'], yhat, label=rate)

ax.set(xlabel='Iteration', ylabel='Error Rate (%)',
       title='Digit Recognition - Linear Classifier')
ax.legend(title='Learning Rate')
fig.savefig('test.png')
plt.show()