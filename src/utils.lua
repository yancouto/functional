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

local function deepEquals(a, b)
	if type(a) ~= type(b) then return false end
	if type(a) == 'table' then
		for k, v in pairs(a) do
			if not deepEquals(a[k], b[k]) then
				return false
			end
		end
		for k, v in pairs(b) do
			if a[k] == nil then
				return false
			end
		end
		return true
	else
		return a == b
	end
end

local function exhaust(fn)
	local co = coroutine.create(fn)
	local function aux(ok, ...)
		if not ok then
			error(...)
		else
			return select('#', ...), ...
		end
	end
	local wrap
	repeat
		wrap = { aux(coroutine.resume(co)) }
	until coroutine.status(co) == 'dead'
	return unpack(wrap, 2, wrap[1] + 1)
end

return {
	shallowCopy      = shallowCopy,
	deepCopyAndShift = deepCopyAndShift,
	deepEquals       = deepEquals,
	exhaust          = exhaust,
}
