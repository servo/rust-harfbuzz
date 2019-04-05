#!/bin/bash

# If the timestamp on these files is incorrect (because git does not preserve timestamps)
# then the configure script will incorrectly try to run automake commands to regenerate them.
# We `touch` these files to prevent this.
AUTOMAKE_FILES="configure \
                configure.ac \
                m4/*.m4 \
                aclocal.m4 \
                configure \
                config.h.in \
                Makefile.* \
                */Makefile.* \
                */*/Makefile.* \
                */*/*/Makefile.* \
                */*/*/*/Makefile.* \
                gtk-doc.make"

cd $(git rev-parse --show-toplevel)/harfbuzz-sys/harfbuzz
touch $AUTOMAKE_FILES
