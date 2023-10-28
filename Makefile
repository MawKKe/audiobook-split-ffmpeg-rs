.DEFAULT_GOAL := demo 

out/split-demo:
	mkdir -p $@


demo: | out/split-demo
	rm -rf out/split-demo
	cargo run -q -- -i src/ffmpeg_split/testdata/beep.m4a -o out/split-demo

.PHONY: test
