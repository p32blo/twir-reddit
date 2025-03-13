run : markdown 

data:
  mkdir -p data

feed: data
  curl --progress-bar https://this-week-in-rust.org/atom.xml > data/atom.xml
  cargo run --release --bin feed -- data/atom.xml > data/atom-twir.html

html url: data
  curl --progress-bar {{url}} --output data/twir.html

extract: feed
  cargo run --release --bin extract -- data/atom-twir.html > data/url.txt

markdown: extract
  cargo run --release --bin markdown -- data/url.txt

clean:
  rm -rf data/