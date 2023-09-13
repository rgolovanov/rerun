# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer20Ext" class in "affix_fuzzer20_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    BaseDelegatingExtensionArray,
    BaseDelegatingExtensionType,
)

__all__ = ["AffixFuzzer20Array", "AffixFuzzer20Type"]


class AffixFuzzer20Type(BaseDelegatingExtensionType):
    _TYPE_NAME = "rerun.testing.components.AffixFuzzer20"
    _DELEGATED_EXTENSION_TYPE = datatypes.AffixFuzzer20Type


class AffixFuzzer20Array(BaseDelegatingExtensionArray[datatypes.AffixFuzzer20ArrayLike]):
    _EXTENSION_NAME = "rerun.testing.components.AffixFuzzer20"
    _EXTENSION_TYPE = AffixFuzzer20Type
    _DELEGATED_ARRAY_TYPE = datatypes.AffixFuzzer20Array


AffixFuzzer20Type._ARRAY_TYPE = AffixFuzzer20Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer20Type())