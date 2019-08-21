.SILENT: clean format 9ccrs
.PHONY: test clean
SRCS=$(wildcard *.rs)

9ccrs: clean format
	rustc -o 9ccrs $(SRCS)
	
test: 9ccrs
	./test.sh

format:
	rustfmt *.rs

clean:
	rm -f 9ccrs *.o *~ tmp*

