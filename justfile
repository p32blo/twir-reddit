all url: (markdown url)

html url: 
  mkdir -p data
  curl --progress-bar {{url}} --output data/twir.html

extract url: (html url)
  cargo run  --bin extract --release -- data/twir.html > data/url.txt

markdown url: (extract url)
  cargo run  --bin markdown --release  -- data/url.txt

clean:
  rm -rf data/