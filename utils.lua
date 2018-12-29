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

return {
    shallowCopy      = shallowCopy,
    deepCopyAndShift = deepCopyAndShift,
}
