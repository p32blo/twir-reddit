import sqlite3
from pathlib import Path

DB = '/home/andre/Desktop/reddit-twir/rss.sqlite'

with sqlite3.connect(DB) as con:
    con.row_factory = sqlite3.Row
    cur = con.cursor()
    rows = cur.execute('SELECT md5, published, title, post FROM rss')

    for row in rows:
        md5 = row['md5']
        published = row['published']
        title = row['title']
        post = row['post']

        with open(Path('output') / f'{title}-{md5}', 'w') as f:
            f.write(post)
