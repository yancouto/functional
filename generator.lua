local function generate(root)
    local buffer = {}

    local function recGen(node)
        if not node then return end
        if node.type == 'identifier' then
            table.insert(buffer, node.name)
        elseif node.type == 'function' then
            table.insert(buffer, '(')
            table.insert(buffer, node.var)
            table.insert(buffer, ': ')
            recGen(node.body)
            table.insert(buffer, ')')
        elseif node.type == 'apply' then
            table.insert(buffer, '(')
            recGen(node.left)
            table.insert(buffer, ' ')
            recGen(node.right)
            table.insert(buffer, ')')
        else
            error "Unknown node type."
        end
    end

    recGen(root)
    return table.concat(buffer)
end

return generate
