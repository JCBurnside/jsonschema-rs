import json
import sys
from functools import partial

import fastjsonschema
import jsonschema
import pytest

if sys.implementation.name != "pypy":
    import jsonschema_rs
else:
    jsonschema_rs = None


def load_json(filename):
    with open(filename) as fd:
        return json.load(fd)


BIG_SCHEMA = load_json("../../jsonschema/benches/swagger.json")
BIG_INSTANCE = load_json("../../jsonschema/benches/kubernetes.json")
SMALL_SCHEMA = load_json("../../jsonschema/benches/small_schema.json")
SMALL_INSTANCE_VALID = [9, "hello", [1, "a", True], {"a": "a", "b": "b", "d": "d"}, 42, 3]


@pytest.fixture(params=[True, False], ids=("compiled", "raw"))
def is_compiled(request):
    return request.param


if jsonschema_rs is not None:
    variants = ["jsonschema-rs-is-valid", "jsonschema-rs-validate", "jsonschema", "fastjsonschema"]
else:
    variants = ["jsonschema", "fastjsonschema"]


DEFAULT_BENCHMARK_CONFIG = {"iterations": 10, "rounds": 10, "warmup_rounds": 10}


@pytest.fixture(params=variants)
def variant(request):
    return request.param


@pytest.fixture
def args(request, variant, is_compiled):
    schema, instance = request.node.get_closest_marker("data").args
    if variant == "jsonschema-rs-is-valid":
        if is_compiled:
            return jsonschema_rs.JSONSchema(schema, with_meta_schemas=True).is_valid, instance
        else:
            return partial(jsonschema_rs.is_valid, with_meta_schemas=True), schema, instance
    if variant == "jsonschema-rs-validate":
        if is_compiled:
            return jsonschema_rs.JSONSchema(schema, with_meta_schemas=True).validate, instance
        else:
            return partial(jsonschema_rs.validate, with_meta_schemas=True), schema, instance
    if variant == "jsonschema":
        if is_compiled:
            return jsonschema.validators.validator_for(schema)(schema).is_valid, instance
        else:
            return jsonschema.validate, instance, schema
    if variant == "fastjsonschema":
        if is_compiled:
            return fastjsonschema.compile(schema, use_default=False), instance
        else:
            return partial(fastjsonschema.validate, use_default=False), schema, instance


@pytest.mark.data(True, True)
@pytest.mark.benchmark(group="boolean")
def test_boolean(benchmark, args):
    benchmark(*args)


@pytest.mark.data({"minimum": 10}, 10)
@pytest.mark.benchmark(group="minimum")
def test_minimum(benchmark, args):
    benchmark(*args)


@pytest.mark.data(SMALL_SCHEMA, SMALL_INSTANCE_VALID)
@pytest.mark.benchmark(group="small")
def test_small_schema(benchmark, args):
    benchmark(*args)


@pytest.mark.data(BIG_SCHEMA, BIG_INSTANCE)
@pytest.mark.benchmark(group="big")
def test_big_schema(benchmark, args):
    benchmark(*args)
