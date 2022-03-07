import json
import gzip
from urllib.request import urlopen

def save_file(url, filename):
    print(f'Downloading {url}')
    with urlopen(url) as f:
        data = f.read()
    print(f'Saving to {filename}')
    with open(filename, 'wb+') as f:
        f.write(gzip.decompress(data))

if __name__ == '__main__':
    with open('files/dataset.json') as f:
        datasets = json.load(f)

    for file, url in datasets['mnist'].items():
        save_file(url, filename=f'files/mnist-{file}')