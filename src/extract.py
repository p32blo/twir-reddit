import sys

from bs4 import BeautifulSoup


def extract_urls(content):
    soup = BeautifulSoup(content, 'html.parser')

    for title in soup.find_all('h3'):
        print(f'##{title.text}')
        for link in title.find_next('ul').find_all('a'):
            url = link.get('href')
            yield url


if __name__ == '__main__':
    filename = sys.argv[1]
    with open(filename) as f:
        for url in extract_urls(filename):
            print(url)
