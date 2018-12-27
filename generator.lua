local function generate(root)
    local buffer = {}
    local has_def = {}
    local def = {}

    local function recGen(node, level)
        if not node then return end
        if node.type == 'identifier' then
            local name = node.name
            if type(name) == 'number' then
                name = def[level - 1 - name]
            end
            table.insert(buffer, name)
        elseif node.type == 'function' then
            table.insert(buffer, '(')
            local v = node.hint
            if has_def[v] then
                for i = 2, 10000 do
                    if not has_def[v .. i] then
                        v = v .. i
                        break
                    end
                end
            end
            table.insert(buffer, v)
            table.insert(buffer, ': ')
            has_def[v] = true
            def[level] = v
            recGen(node.body, level + 1)
            def[level] = nil
            has_def[v] = false
            table.insert(buffer, ')')
        elseif node.type == 'apply' then
            table.insert(buffer, '(')
            recGen(node.left, level)
            table.insert(buffer, ' ')
            recGen(node.right, level)
            table.insert(buffer, ')')
        else
            error "Unknown node type."
        end
    end

    recGen(root, 0)
    return table.concat(buffer)
end

return generate
