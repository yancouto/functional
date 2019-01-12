local parse = require "parser"
local gen   = require "generator"
local utils = require "utils"

local function substitute(node, var_name, val)
	if not node then return nil end
	if node.type == 'identifier' then
		if node.name == var_name then
			return utils.deepCopyAndShift(val, var_name, 0)
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

local function interpret(str, known_cts)
	local all = { parse(str, known_cts) }
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
