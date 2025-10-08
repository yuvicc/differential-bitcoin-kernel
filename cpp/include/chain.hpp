#pragma once

#include "types.hpp"

namespace btck {

class ChainParameters : public Handle<btck_ChainParameters, btck_chain_parameters_copy, btck_chain_parameters_destroy>
{

};


class ContextOptions : UniqueHandle<btck_ContextOptions, btck_context_options_destroy>
{

};

class Context : public Handle<btck_Context, btck_context_copy, btck_context_destroy>
{

};

class ChainstateManagerOptions : UniqueHandle<btck_ChainstateManagerOptions, btck_chainstate_manager_options_destroy>
{

};

class ChainstateManager : UniqueHandle<btck_ChainstateManager, btck_chainstate_manager_destroy>
{

};

class ChainView : public View<btck_Chain>
{

};

}