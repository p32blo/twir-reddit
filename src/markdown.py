import html
import sys
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup

filename = sys.argv[1]
URLS = []

with open(filename) as f:
    URLS = [line.strip().rstrip('/') for line in f]


def urls_iterator(urls):
    for url in urls:
        if not url.startswith('##'):
            yield url


print(
    """
# TWIR @ Reddit

Hey everyone, here you can follow the r/rust comment threads of articles featured in TWIR (This Week in Rust).
I've always found it helpful to search for additional insights in the comment section here
and I hope you can find it helpful too.
Enjoy !
"""
)


def extract_post(post):
    data = post['data']

    title = data['title']
    url = data['url']
    permalink = data['permalink']
    num_comments = data['num_comments']
    ups = data['ups']
    downs = data['downs']
    score = data['score']

    links = []
    selftext_html = post['data']['selftext_html']
    if selftext_html is not None:
        selftext_html = html.unescape(selftext_html)
        soup = BeautifulSoup(selftext_html, 'html.parser')

        for link in soup.find_all('a'):
            href = link.get('href')
            links.append(href)

    return {
        'title': html.unescape(title),
        'url': url,
        'links': links,
        'permalink': permalink,
        'num_comments': num_comments,
        'ups': ups,
        'downs': downs,
        'score': score,
    }


def call(after=None):
    url = 'http://www.reddit.com/r/rust/new.json'

    params = {'sort': 'new'}

    if after is not None:
        params['after'] = after

    response = requests.get(url, headers={'User-Agent': 'Rust TWIR/0.0.1'}, params=params)
    return response


@dataclass
class Result:
    title: str
    url: str
    num_comments: int
    ups: int
    downs: int
    score: int


RESULTS_MAP = {}

after = None

for _ in range(15):
    response = call(after=after)
    # print(response.text)
    result = response.json()
    after = result['data']['after']

    for post in result['data']['children']:
        post_content = extract_post(post)

        links = post_content['links'] + [post_content['url']]
        links = (link.rstrip('/') for link in links)

        # print(f'{[l.rstrip("/") for l in links]=}')
        for link in links:
            for url in urls_iterator(URLS):
                if link == url:
                    RESULTS_MAP[url] = Result(
                        title=post_content['title'],
                        url=f"http://www.reddit.com{post_content['permalink']}",
                        num_comments=post_content['num_comments'],
                        ups=post_content['ups'],
                        downs=post_content['downs'],
                        score=post_content['score'],
                    )

        # else:
        #   print('.', end='')
        #    sys.stdout.flush()
        #   print(f"XXXXXXXX - {post_content['title']} - http://www.reddit.com{post_content['permalink']}")
        # pprint(URLS)
# print()


def print_buffer(buffer):
    if len(buffer) > 1:
        print()
        for line in buffer:
            print(line)


BUFFER = []
for url in URLS:
    if url.startswith('##'):
        print_buffer(BUFFER)
        BUFFER = [url]

    elif url in RESULTS_MAP:
        result = RESULTS_MAP[url]
        BUFFER.append(
            f"- [{result.title}]({result.url})"
            f" `↑{result.score} | {result.num_comments} comment{'s' if result.num_comments > 1 else ''}`"
        )

print_buffer(BUFFER)
