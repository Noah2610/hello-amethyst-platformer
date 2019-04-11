nmap <leader>r :!bin/run<CR>
nmap <leader>R :!RUST_BACKTRACE=1 bin/run 2>&1 \| grep -A 1 'hello_amethyst_platformer'<CR>
