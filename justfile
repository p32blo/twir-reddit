all url: (markdown url)

html url: 
  wget --quiet {{url}} --output-document twir.html

extract url: (html url)
  cd twir && cargo run --quiet --bin extract --release -- ../twir.html > ../url.txt

markdown url: (extract url)
  cd twir && cargo run --quiet --bin markdown --release  -- ../url.txt
