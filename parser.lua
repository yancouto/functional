local tokenizer = require "tokenizer"

local function parse(str)
    local lex = tokenizer(str)
    local token = lex()
    if not token then error { msg = "Can't be empty" } end
    -- def.a = vector with levels a was the variable
    -- in a function declaration
    local def = {}

    local function nxt()
        token = token and lex()
    end

    local function recParse(level)
        local root = nil
        while true do
            local node
            if not token or token.value == ')' then
                return root
            elseif token.value == '(' then
                nxt()
                node = recParse(level)
                if not token or token.value ~= ')' then error { msg = "Missing close parenthesis" } end
                nxt()
            elseif token.value == ':' then
                error { msg = "Invalid ':'", loc = token.loc }
            else -- text
                local name, loc = token.value, token.loc
                nxt()
                if token and token.value == ':' then
                    nxt()
                    if name ~= name:match("[a-z]") then
                        error {
                            msg = "Invalid variable name '" .. name .. "'. Must be single lowercase letter",
                            loc = loc
                        }
                    end
                    def[name] = def[name] or {}
                    table.insert(def[name], level)
                    node = {
                        type = 'function',
                        -- Use this var name if possible
                        hint = name,
                        body = recParse(level + 1)
                    }
                    assert(table.remove(def[name]) == level)
                else
                    -- variable
                    if name == name:match("[a-z]") then
                        if not def[name] or not def[name][1] then
                            error {
                                msg = "Unbound var '" .. name .. "'",
                                loc = loc
                            }
                        end
                        name = level - def[name][#def[name]] - 1
                    else
                        -- check if you're allowed to use that constant
                    end
                    node = {
                        type = 'identifier',
                        name = name
                    }
                end
            end
            if root then
                root = { type = 'apply', left = root, right = node }
            else
                root = node
            end
        end
    end

    local root = recParse(0)
    if token then
        error {
            msg = "Extra closing parenthesis",
            loc = token.loc
        }
    end
    return root
end

return parse
