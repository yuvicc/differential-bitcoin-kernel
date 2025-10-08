#pragma once

#include "types.hpp"

namespace btck {

class BlockHash : public Handle<btck_BlockHash, btck_block_hash_copy, btck_block_hash_destroy>
{
public:
    explicit BlockHash(const std::array<std::byte, 32>& hash)
        : Handle{btck_block_hash_create(reinterpret_cast<const unsigned char*>(hash.data()))} {}
    
    explicit BlockHash(btck_BlockHash* hash)
        : Handle{hash} {}
    
    bool operator==(const BlockHash& other) const
    {
        return btck_block_hash_equals(get(), other.get()) != 0;
    }

    bool operator!=(const BlockHash* other) const
    {
        return btck_block_hash_equals(get(), other.get()) == 0;
    }

    std::array<std::byte, 32> ToBytes() const
    {
        std::array<std::byte, 32> hash;
        btck_block_hash_to_bytes(get(), reinterpret_cast<unsigned char*>(hash.data()));
        return hash;
    }
}

class Block : public Handle<btck_Block, btck_block_copy, btck_block_destroy>
{
public:
    Block(const std::span<const std::byte> raw_block)
        : Handle{btck_block_create(raw_block.data(), raw_block.size())}
    {
    }

    Block(btck_Block* block) : Handle{block} {}

    TransactionView GetTransaction(size_t index) const
    {
        return TransactionView{btck_block_get_transaction_at(get(), index)};
    }

    auto Transactions() const
    {
        return Range<Block, &Block::CountTransactions, &Block::GetTransaction>{*this};
    }

    BlockHash GetHash() const
    {
        return BlockHash{btck_block_get_hash(get())};
    }

    std::vector<std::byte> ToBytes() const
    {
        return write_bytes(get(), btck_block_to_bytes);
    }

    friend class ChainMan;
}

class BlockTreeEntry : public View<btck_BlockTreeEntry>
{
public:


}



class BlockSpentOutputs;
class BlockValidationState;



}