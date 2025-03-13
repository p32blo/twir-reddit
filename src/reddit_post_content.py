import html
import sys
from dataclasses import dataclass

import requests
from bs4 import BeautifulSoup
from pprint import pprint as print


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

    response = requests.get(url,
                            headers={'User-Agent': 'Rust TWIR/0.0.1'},
                            params=params)
    return response


@dataclass
class Result:
    title: str
    url: str
    num_comments: int
    ups: int
    downs: int
    score: int


if __name__ == "__main__":
    response = call()
    print(
        list(
            filter(
                lambda x: "This Week in Rust".lower() in x,
                (child['data']['title'].lower()
                 for child in response.json()['data']['children']),
            )))
