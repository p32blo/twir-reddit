import requests
from bs4 import BeautifulSoup
import sys

# exclude = [
#     "https://this-week-in-rust.org/",
#     "https://this-week-in-rust.org/blog/2023/05/24/this-week-in-rust-496/",
#     "https://www.rust-lang.org/",
#     "https://twitter.com/ThisWeekInRust",
#     "https://mastodon.social/@thisweekinrust",
#     "https://github.com/rust-lang/this-week-in-rust",
#     "https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md",
#     "https://github.com/rust-lang/this-week-in-rust",
#     "https://github.com/rust-lang/this-week-in-rust/pulls",
#     "https://www.reddit.com/r/rust/comments/13r0c4c/this_week_in_rust_496/",
#     "https://this-week-in-rust.org/blog/2023/05/17/this-week-in-rust-495/",
#     "https://this-week-in-rust.org/blog/archives/index.html",
#     "https://this-week-in-rust.org/atom.xml",
#     "https://this-week-in-rust.org/rss.xml",
#     "https://this-week-in-rust.org/pages/privacy-policy.html",
#     "http://creativecommons.org/licenses/by-sa/4.0/",
# ]

url = sys.argv[1]
reqs = requests.get(url)
soup = BeautifulSoup(reqs.text, 'html.parser')

urls = []
for link in soup.find_all('a'):
    url = link.get('href')
    #if not url in exclude:
    print(url)
