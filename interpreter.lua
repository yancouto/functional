local parse = require "parser"
local gen = require "generator"

local function shallowCopy(node)
    local cp = {}
    for k, v in pairs(node) do
        cp[k] = v
    end
    return cp
end

local function deepCopyAndShift(node, shift, cur_level)
    local cp = shallowCopy(node)
    if node.type == 'identifier' then
        if type(cp.name) == 'number' and cp.name > cur_level then
            cp.name = cp.name + shift
        end
    elseif node.type == 'function' then
        cp.body = deepCopyAndShift(cp.body, shift, cur_level + 1)
    elseif node.type == 'apply' then
        cp.left = deepCopyAndShift(cp.left, shift, cur_level)
        cp.right = deepCopyAndShift(cp.right, shift, cur_level)
    else
        error 'Unknown node type'
    end
    return cp
end

local function substitute(node, var_name, val)
    if not node then return nil end
    if node.type == 'identifier' then
        if node.name == var_name then
            return deepCopyAndShift(val, var_name, 0)
        end
    elseif node.type == 'function' then
        node.body = substitute(node.body, var_name + 1, val)
    elseif node.type == 'apply' then
        node.left = substitute(node.left, var_name, val)
        node.right = substitute(node.right, var_name, val)
    else
        error 'Unknown node type'
    end
    return node
end

local function interpret(str)
    local all = { parse(str) }
    coroutine.yield(all[1])

    local function read(obj, k)
        local node = obj[k]
        if node and node.type == 'apply' then
            read(node, 'left')
            read(node, 'right')
            if node.left and node.left.type == 'function' then
                obj[k] = substitute(node.left.body, 0, node.right)
                coroutine.yield(all[1])
                read(obj, k)
            end
        end
    end

    read(all, 1)
    return all[1]
end

return interpret
