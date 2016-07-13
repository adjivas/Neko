LATEXCMD="xelatex --shell-escape --enable-write18 %O %S"
LATEXMKOPTS=-pdflatex=$(LATEXCMD) -pdf -dvi- -ps- -bibtex

.PHONY: default info all a4 c5 mclean-a4 mclean-c5 clean-a4 clean-c5
.SILENT: info clean-a4 clean-c5 mclean-a4 mclean-c5 dclean-a4 dclean-c5

default: info

info:
	echo "Please make a specific target (a4 or c5), or explicitly run \"make all\"."

all: a4 c5

a4:
	latexmk $(LATEXMKOPTS) -jobname=latexbok-a4 -recorder -f doc.tex

c5:
	latexmk $(LATEXMKOPTS) -jobname=latexbok-c5 -recorder -f doc.tex

clean: clean-a4 clean-c5

clean-a4: mclean-a4 dclean-a4 
	rm -v -f latexbok-a4-*.cpt *.fls

clean-c5: mclean-c5 dclean-c5
	rm -v -f latexbok-c5-*.cpt *.fls

mclean-a4:
	latexmk -c -f -bibtex -jobname=latexbok-a4 -recorder a4.tex

mclean-c5:
	latexmk -c -f -bibtex -jobname=latexbok-c5 -recorder c5.tex

dclean-a4:
	rm -v -f latexbok-a4-dot2tex-fig*.dot
	rm -v -f latexbok-a4-dot2tex-fig*.tex

dclean-c5:
	rm -v -f latexbok-c5-dot2tex-fig*.dot
	rm -v -f latexbok-c5-dot2tex-fig*.tex
