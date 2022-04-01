#!/bin/sh

# https://qiita.com/shinichi-takii/items/e90dcf7550ef13b047b5

cd bindings/python3

pip3 install -r requirements.txt

# 本番アップロード
twine upload --repository pypi dist/*
