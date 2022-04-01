#!/bin/bash
set -ex

rm -f -r chanoma.egg-info/* dist/*

for PYBIN in /opt/python/cp{37,38,39,310}*/bin; do
  "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
  "${PYBIN}/python" setup.py sdist
  "${PYBIN}/python" setup.py bdist_wheel
done

for whl in dist/*-linux_x86_64.whl; do
  auditwheel repair "$whl" -w dist/ --plat manylinux_2_28_x86_64
  rm $whl
done
