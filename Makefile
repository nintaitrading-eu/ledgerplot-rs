# ledgerplot
# See LICENSE file for copyright and license details.

include config.mk

SRC = ${BUILDDIR}/ledgerplot

all: ledgerplot

ledgerplot:
	@echo cargo build --release
	cargo build --release

clean:
	@echo Cargo clean...
	cargo clean

dist: clean
	@echo creating dist tarball
	@mkdir -p ledgerplot-${VERSION}
	@cp -R LICENSE.txt Makefile README.adoc config.mk \
	ledgerplot.1 ${SRC} ledgerplot-${VERSION}
	@tar -cf ledgerplot-${VERSION}.tar ledgerplot-${VERSION}
	@gzip ledgerplot-${VERSION}.tar
	@rm -rf ledgerplot-${VERSION}

install:
	@echo installing executable file to ${DESTDIR}${PREFIX}/bin
	@mkdir -p ${DESTDIR}${PREFIX}/bin
	@cp -f ${BUILDDIR}/ledgerplot ${DESTDIR}${PREFIX}/bin
	@chmod 755 ${DESTDIR}${PREFIX}/bin/ledgerplot
	@mkdir -p ${DESTDIR}${SHARE}/ledgerplot
	@echo installing manual page to ${DESTDIR}${MANPREFIX}/man1
	@mkdir -p ${DESTDIR}${MANPREFIX}/man1
	@sed "s/VERSION/${VERSION}/g" < ledgerplot.1 > ${DESTDIR}${MANPREFIX}/man1/ledgerplot.1
	@chmod 644 ${DESTDIR}${MANPREFIX}/man1/ledgerplot.1

uninstall:
	@echo removing executable file from ${DESTDIR}${PREFIX}/bin
	@rm -f ${DESTDIR}${PREFIX}/bin/ledgerplot
	@echo removing data in /usr/local/share from ${DESTDIR}${SHARE}/ledgerplot
	@rm -rf ${DESTDIR}${SHARE}/ledgerplot
	@echo removing manual page from ${DESTDIR}${MANPREFIX}/man1
	@rm -f ${DESTDIR}${MANPREFIX}/man1/ledgerplot.1

.PHONY: all options clean dist install uninstall
