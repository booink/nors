# nors

nors is a counting the number of rows and records in a CSV file.

## Install

```sh
pip install nors
```

## Usage

```python
from nors import count

print(count("10_lines_and_records.csv"))
{'lines': 10, 'csv_records': 10}
```

## Development

```sh
docker-compose build nors
docker-compose run --rm nors bash
cd /app/bindings/python3
python3 ./setup.py develop
./run-test.sh
```

## Build

```sh
docker-compose run --rm build-wheels-for-python ./build-wheels.sh
```

## Publish

```sh
docker-compose run --rm nors ./bindings/python3/publish_to_pypi.sh
```
