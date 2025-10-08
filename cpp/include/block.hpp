#pragma once

#include <array>
#include <functional>
#include <memory>
#include <optional>
#include <span>
#include <stdexcept>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

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
    BlockTreeEntry(const btck_BlockTreeEntry* entry)
        : View{entry}
    {
    }

    std::optional<BlockTreeEntry> GetPrevious() const
    {
        auto entry{btck_block_tree_entry_get_previous(get())};
        if (!entry) return std::nullopt;
        return entry;
    }

    int32_t GetHeight() const
    {
        return btck_block_tree_entry_get_height(get());
    }

    BlockHash GetHash() const
    {
        return BlockHash{btck_block_tree_entry_get_block_hash(get())};
    }

    friend class ChainMan;
    friend class Chain;
}

class BlockSpentOutputs : Handle<btck_BlockSpentOutputs, btck_block_spent_outputs_copy, btck_block_spent_outputs_destroy>
{
public:
    BlockSpentOutputs(btck_BlockSpentOutputs* block_spent_outputs)
        : Handle{block_spent_outputs}
    {
    }

    size_t Count() const
    {
        return btck_block_spent_outputs_count(get());
    }

    TransactionSpentOutputsView GetTxSpentOutputs(size_t tx_undo_index) const
    {
        return TransactionSpentOutputsView{btck_block_spent_outputs_get_transaction_spent_outputs_at(get(), tx_undo_index)};
    }

    auto TxsSpentOutputs() const
    {
        return Range<BlockSpentOutputs, &BlockSpentOutputs::Count, &BlockSpentOutputs::GetTxSpentOutputs>{*this};
    }


}



class BlockValidationState
{
private:
    const btck_BlockValidationState* m_state;

public:
    BlockValidationState(const btck_BlockValidationState* state) : m_state{state} {}

    BlockValidationState(const BlockValidationState&) = delete;
    BlockValidationState& operator=(const BlockValidationState&) = delete;
    BlockValidationState(const BlockValidationState&&) = delete;
    BlockValidationState& operator=(const BlockValidationState&&) = delete;

    ValidationMode GetValidationMode() const
    {
        return static_cast<ValidationMode>(btck_block_validation_state_get_validation_mode(m_state));
    }

    BlockValidationResult GetBlockValidationResult() const
    {
        return static_cast<BlockValidationResult>(btck_block_validation_state_get_block_validation_result(m_state));
    }
};
}