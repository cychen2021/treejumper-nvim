-- TreeJumper Neovim Plugin - Main Entry Point

local M = {}

--- Setup the plugin with optional configuration
---@param opts table|nil Optional configuration table
function M.setup(opts)
    opts = opts or {}

    -- TODO: Initialize the plugin
    -- - Load native library
    -- - Set up commands
    -- - Set up keymaps
end

--- Jump to the next code node
function M.jump_next()
    -- TODO: Implement next node navigation
end

--- Jump to the previous code node
function M.jump_prev()
    -- TODO: Implement previous node navigation
end

--- Show information about the current node
function M.show_info()
    -- TODO: Show current node information
end

return M
