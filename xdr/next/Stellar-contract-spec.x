// Copyright 2022 Stellar Development Foundation and contributors. Licensed
// under the Apache License, Version 2.0. See the COPYING file at the root
// of this distribution or at http://www.apache.org/licenses/LICENSE-2.0

% #include "xdr/Stellar-types.h"
% #include "xdr/Stellar-contract.h"
namespace stellar
{

enum SpecType
{
    SPEC_TYPE_FUNCTION = 0,
    SPEC_TYPE_U32 = 1,
    SPEC_TYPE_I32 = 2,
    SPEC_TYPE_U64 = 3,
    SPEC_TYPE_I64 = 4,
    SPEC_TYPE_BOOL = 5,
    SPEC_TYPE_SYMBOL = 6,
    SPEC_TYPE_BITSET = 7,
    SPEC_TYPE_STATUS = 8,
    SPEC_TYPE_BINARY = 9,
    SPEC_TYPE_VEC = 10,
    SPEC_TYPE_MAP = 11,
    SPEC_TYPE_TUPLE = 12,
    SPEC_TYPE_STRUCT = 13,
    SPEC_TYPE_UNION = 14
};

union SpecTypeDef switch (SpecType type)
{
case SPEC_TYPE_FUNCTION:
    struct {
        SpecTypeDef argTypes<12>;
        SpecTypeDef *resultType;
    } function;
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
        SpecTypeDef valueTypes<12>;
    } tuple;
case SPEC_TYPE_STRUCT:
    struct {
        struct {
            SCSymbol name;
            SpecTypeDef typ;
        } field<12>;
    } struct;
case SPEC_TYPE_UNION:
    struct {
        SpecTypeDef discriminant;
        SpecTypeDef caseTypes<12>;
    } union;
};

enum SpecEntryType
{
    SPEC_ENTRY_V0 = 0
};

union SpecEntry switch (SpecEntryType type)
{
case SPEC_ENTRY_V0:
    struct {
        SCSymbol name;
        SpecTypeDef typ;
    } v0;
};

}
