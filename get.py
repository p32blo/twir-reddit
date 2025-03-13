import requests


def call(after=None):
    url = 'http://www.reddit.com/r/rust/new.json'

    params = {'sort': 'new'}

    if after is not None:
        params['after'] = after

    response = requests.get(url, headers={'User-Agent': 'Rust TWIR/0.0.1'}, params=params)
    return response


after = None

for _ in range(9):
    response = call(after=after)
    result = response.json()
    after = result['data']['after']
    print(response.text)
