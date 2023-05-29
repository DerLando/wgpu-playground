# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

run FRAGMENT_SHADER_PATH:
	cargo run --release -- {{FRAGMENT_SHADER_PATH}}
