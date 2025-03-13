import hashlib
import sqlite3
import sys
from datetime import datetime

import feedparser

FEED = 'https://this-week-in-rust.org/rss.xml'
DB = '/home/andre/Desktop/reddit-twir/rss.sqlite'
VERSION = '0.1.0'
SOURCE = sys.argv[1] if len(sys.argv) > 1 else 'TEST'

feed = feedparser.parse(FEED)

with sqlite3.connect(DB) as con:
    cur = con.cursor()
    cur.execute('CREATE TABLE IF NOT EXISTS rss(md5 PRIMARY KEY, update_time, id, published, post);')
    cur.execute('CREATE TABLE IF NOT EXISTS _log(id INTEGER PRIMARY KEY, timestamp, version, source);')

    now = datetime.now()

    for post in feed.entries:
        # with open(f'rss/"{post.id}-{post.published}.html"', "w") as f:
        # with open(f"rss/{post.id}-{post.published}.html", "w") as f:
        cur.execute(
            'INSERT OR REPLACE INTO rss(md5, update_time, id, published, post) VALUES (?, ?, ?, ?, ?);',
            (
                hashlib.md5(post.summary.encode()).hexdigest(),
                now,
                post.id,
                post.published,
                post.summary,
            ),
        )

    cur.execute(
        'INSERT INTO _log(timestamp, version, source) VALUES (?, ?, ?);',
        (
            now,
            VERSION,
            SOURCE,
        ),
    )
