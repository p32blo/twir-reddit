import sys

import requests
from extract import extract_urls

if __name__ == '__main__':
    url = sys.argv[1]
    reqs = requests.get(url)

    for url in extract_urls(reqs.text):
        print(url)
