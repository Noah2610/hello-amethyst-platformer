nmap <leader>r :!cargo +nightly-2019-03-01 run --features amethyst/nightly<CR>
nmap <leader>R :!RUST_BACKTRACE=1 cargo +nightly-2019-03-01 run --features amethyst/nightly 2>&1 \| grep -A 1 'hello_amethyst_platformer'<CR>
nmap <leader>t :!cargo test<CR>
