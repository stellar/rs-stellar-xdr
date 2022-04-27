// Copyright 2022 Stellar Development Foundation and contributors. Licensed
// under the Apache License, Version 2.0. See the COPYING file at the root
// of this distribution or at http://www.apache.org/licenses/LICENSE-2.0

% #include "xdr/Stellar-types.h"
% #include "xdr/Stellar-ledger-entries.h"
namespace stellar
{
/*
 * Smart Contracts deal in SCVals. These are a (dynamic) disjoint union
 * between several possible variants, to allow storing generic SCVals in
 * generic data structures and passing them in and out of languages that
 * have simple or dynamic type systems.
 *
 * SCVals are (in WASM's case) stored in a tagged 64-bit word encoding. Most
 * signed 64-bit values in Stellar are actually signed positive values
 * (sequence numbers, timestamps, amounts), so we don't need the high bit
 * and can get away with 1-bit tagging and store them as "u63", separate
 * from everything else.
 *
 * We actually reserve the low _four_ bits, leaving 3 bits for 8 cases of
 * "non u63 values", some of which has some substructure of their own.
 *
 *    0x_NNNN_NNNN_NNNN_NNNX  - u63 for any even X
 *    0x_0000_000N_NNNN_NNN1  - u32
 *    0x_0000_000N_NNNN_NNN3  - i32
 *    0x_NNNN_NNNN_NNNN_NNN5  - static: void, true, false, ...
 *    0x_IIII_IIII_TTTT_TTT7  - object: 32-bit index I, 28-bit type code T
 *    0x_NNNN_NNNN_NNNN_NNN9  - symbol: up to 10 6-bit identifier characters
 *    0x_NNNN_NNNN_NNNN_NNNb  - bitset: up to 60 bits
 *    0x_CCCC_CCCC_TTTT_TTTd  - status: 32-bit code C, 28-bit type code T
 *    0x_NNNN_NNNN_NNNN_NNNf  - reserved
 *
 * Up here in XDR we have variable-length tagged disjoint unions but no
 * bit-level packing, so we can be more explicit in their structure, at the
 * cost of spending more than 64 bits to encode many cases, and also having
 * to convert. It's a little non-obvious at the XDR level why there's a
 * split between SCVal and SCObject given that they are both immutable types
 * with value semantics; but the split reflects the split that happens in
 * the implementation, and marks a place where different implementations of
 * immutability (CoW, structural sharing, etc.) will likely occur.
 */

// A symbol is up to 10 chars drawn from [a-zA-Z0-9_], which can be packed
// into 60 bits with a 6-bit-per-character code, usable as a small key type
// to specify function, argument, tx-local environment and map entries
// efficiently.
typedef string SCSymbol<10>;

enum SCValType
{
    SCV_U63 = 0,
    SCV_U32 = 1,
    SCV_I32 = 2,
    SCV_STATIC = 3,
    SCV_OBJECT = 4,
    SCV_SYMBOL = 5,
    SCV_BITSET = 6,
    SCV_STATUS = 7
};

% struct SCObject;

enum SCStatic
{
    SCS_VOID = 0,
    SCS_TRUE = 1,
    SCS_FALSE = 2
};

enum SCStatusType
{
    SST_OK = 0,
    SST_UNKNOWN_ERROR = 1
    // TODO: add more
};

union SCStatus switch (SCStatusType type)
{
case SST_OK:
    void;
case SST_UNKNOWN_ERROR:
    uint32 unknownCode;
};

union SCVal switch (SCValType type)
{
case SCV_U63:
    uint64 u63;
case SCV_U32:
    uint32 u32;
case SCV_I32:
    int32 i32;
case SCV_STATIC:
    SCStatic ic;
case SCV_OBJECT:
    SCObject* obj;
case SCV_SYMBOL:
    SCSymbol sym;
case SCV_BITSET:
    uint64 bits;
case SCV_STATUS:
    SCStatus status;
};

enum SCObjectType
{
    // We have a few objects that represent non-stellar-specific concepts
    // like general-purpose maps, vectors, strings, numbers, blobs, etc.

    SCO_BOX = 0,
    SCO_VEC = 1,
    SCO_MAP = 2,
    SCO_U64 = 3,
    SCO_I64 = 4,
    SCO_STRING = 5,
    SCO_BINARY = 6,
    SCO_BIGINT = 7,
    SCO_BIGRAT = 8,

    // Followed by a potentially much longer list of object types
    // corresponding to any XDR ledger types users might want to manipulate
    // directly. Separate type codes are required for composite types and
    // each of their members, if it is reasonable for a user to want to hold
    // a handle to the member separately from the composite type.

    SCO_LEDGERKEY = 9,
    SCO_OPERATION = 10,
    SCO_OPERATION_RESULT = 11,
    SCO_TRANSACTION = 12,
    SCO_ASSET = 13,
    SCO_PRICE = 14,
    SCO_ACCOUNTID = 15

    // TODO: add more
};

struct SCMapEntry
{
    SCVal key;
    SCVal val;
};

typedef SCVal SCVec<>;
typedef SCMapEntry SCMap<>;

struct SCBigInt
{
    bool positive;
    opaque magnitude<>;
};

struct SCBigRat
{
    bool positive;
    opaque numerator<>;
    opaque denominator<>;
};

% struct Transaction;
% struct Operation;
% struct OperationResult;

union SCObject switch (SCObjectType type)
{
case SCO_BOX:
    SCVal box;
case SCO_VEC:
    SCVec vec;
case SCO_MAP:
    SCMap map;
case SCO_U64:
    uint64 u64;
case SCO_I64:
    int64 i64;
case SCO_STRING:
    string str<>;
case SCO_BINARY:
    opaque bin<>;
case SCO_BIGINT:
    SCBigInt bi;
case SCO_BIGRAT:
    SCBigRat br;
case SCO_LEDGERKEY:
    LedgerKey* lkey;
case SCO_OPERATION:
    Operation* op;
case SCO_OPERATION_RESULT:
    OperationResult* ores;
case SCO_TRANSACTION:
    Transaction* tx;
case SCO_ASSET:
    Asset asset;
case SCO_PRICE:
    Price price;
case SCO_ACCOUNTID:
    AccountID accountID;
};
}