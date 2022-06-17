// Copyright 2022 Stellar Development Foundation and contributors. Licensed
// under the Apache License, Version 2.0. See the COPYING file at the root
// of this distribution or at http://www.apache.org/licenses/LICENSE-2.0

% #include "xdr/Stellar-types.h"
% #include "xdr/Stellar-contract.h"
namespace stellar
{

enum SpecType
{
    SPEC_TYPE_UNIT = 0,
    SPEC_TYPE_U32 = 1,
    SPEC_TYPE_I32 = 2,
    SPEC_TYPE_U64 = 3,
    SPEC_TYPE_I64 = 4,
    SPEC_TYPE_BOOL = 5,
    SPEC_TYPE_SYMBOL = 6,
    SPEC_TYPE_BITSET = 7,
    SPEC_TYPE_STATUS = 8,
    SPEC_TYPE_BINARY = 9,
    SPEC_TYPE_OPTION = 10,
    SPEC_TYPE_RESULT = 11,
    SPEC_TYPE_VEC = 12,
    SPEC_TYPE_MAP = 13,
    SPEC_TYPE_SET = 14,
    SPEC_TYPE_TUPLE = 15,
    SPEC_TYPE_STRUCT = 16,
    SPEC_TYPE_UNION = 17
};

union SpecTypeDef switch (SpecType type)
{
case SPEC_TYPE_UNIT:
case SPEC_TYPE_U64:
case SPEC_TYPE_I64:
case SPEC_TYPE_U32:
case SPEC_TYPE_I32:
case SPEC_TYPE_BOOL:
case SPEC_TYPE_SYMBOL:
case SPEC_TYPE_BITSET:
case SPEC_TYPE_STATUS:
case SPEC_TYPE_BINARY:
    void;
case SPEC_TYPE_OPTION:
    struct {
        SpecTypeDef valueType;
    } option;
case SPEC_TYPE_RESULT:
    struct {
        SpecTypeDef okType;
        SpecTypeDef errorType;
    } result;
case SPEC_TYPE_VEC:
    struct {
        SpecTypeDef elementType;
    } vec;
case SPEC_TYPE_MAP:
    struct {
        SpecTypeDef keyType;
        SpecTypeDef valueType;
    } map;
case SPEC_TYPE_TUPLE:
    struct {
        SpecTypeDef valueType<12>;
    } tuple;
case SPEC_TYPE_STRUCT:
    struct {
        struct {
            string name<60>;
            SpecTypeDef typ;
        } field<40>;
    } struct;
case SPEC_TYPE_UNION:
    struct {
        struct {
            string name<60>;
            SpecTypeDef armType;
        } case<40>;
    } union;
};

union SpecEntryFunction switch (int v)
{
case 0:
    struct {
        SCSymbol name;
        SpecTypeDef inputTypes<10>;
        SpecTypeDef outputTypes<1>;
    } v0;
};

union SpecEntryType switch (int v)
{
case 0:
    struct {
        string name<60>;
        SpecTypeDef typ;
    } v0;
};

enum SpecEntryKind
{
    SPEC_ENTRY_FUNCTION = 0,
    SPEC_ENTRY_TYPE = 1
};

union SpecEntry switch (SpecEntryKind kind)
{
case SPEC_ENTRY_FUNCTION:
    SpecEntryFunction function;
case SPEC_ENTRY_TYPE:
    SpecEntryType type;
};

}
