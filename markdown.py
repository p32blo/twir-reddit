import requests
from bs4 import BeautifulSoup
import html
import sys
from pprint import pprint
import sys
import time
from dataclasses import dataclass

filename = sys.argv[1]
URLS = []

with open(filename) as f:
    URLS = [line.strip().rstrip('/') for line in f]

pprint(len(URLS))

print("""
# TWIR @ Reddit

Hey everyone, here you can follow the r/rust comment threads of articles featured in TWIR (This Week in Rust). 
I've always found it helpful to search for additional insights in the comment section here and I hope you can find it helpful too. 
Enjoy !
""")

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

    url = "http://www.reddit.com/r/rust/new.json"

    params = {"sort": "new"}

    if after is not None:
        params['after'] = after

    response = requests.get(url,
                            headers={"User-Agent": "Rust TWIR/0.0.1"},
                            params=params)
    # time.sleep(5)
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
    #print(response.text)
    result = response.json()
    after = result['data']['after']

    for post in result['data']['children']:
        post_content = extract_post(post)
          
        links = post_content['links'] + [post_content['url']]
        links = (l.rstrip('/') for l in links)
        
        #print(f'{[l.rstrip("/") for l in links]=}')
        for link in links:
            for url in URLS:
                if link == url:
                    RESULTS_MAP[url] = Result(
                        title=post_content['title'],
                        url=f"http://www.reddit.com{post_content['permalink']}",
                        num_comments=post_content['num_comments'],
                        ups=post_content['ups'],
                        downs=post_content['downs'],
                        score=post_content['score'],
                    )

        #else:
        #   print('.', end='')
        #    sys.stdout.flush()
         #   print(f"XXXXXXXX - {post_content['title']} - http://www.reddit.com{post_content['permalink']}")
            #pprint(URLS)
#print()

for url in URLS:
    if url in RESULTS_MAP:
        result = RESULTS_MAP[url]
        print(f"- [{result.title}]({result.url}) `↑{result.score} | {result.num_comments} comment{'s' if result.num_comments > 1 else ''}`")