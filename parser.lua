local tokenizer = require "tokenizer"

local function parse(str)
    local lex = tokenizer(str)
    local token = lex()
    if not token then error { msg = "Can't be empty" } end

    local function nxt()
        token = token and lex()
    end

    local function recParse()
        local root = nil
        while true do
            local node
            if not token or token.value == ')' then
                return root
            elseif token.value == '(' then
                nxt()
                node = recParse()
                if not token or token.value ~= ')' then error { msg = "Missing close parenthesis" } end
                nxt()
            elseif token.value == ':' then
                error { msg = "Invalid ':'", loc = token.loc }
            else -- text
                local prev = token
                nxt()
                if token and token.value == ':' then
                    nxt()
                    if prev.value ~= prev.value:match("[a-z]") then
                        error{
                            msg = "Invalid variable name '" .. prev.value .. "'. Must be single lowercase letter",
                            loc = prev.loc
                        }
                    end
                    node = {
                        type = 'function',
                        var  = prev.value,
                        body = recParse()
                    }
                else
                    node = {
                        type = 'identifier',
                        name = prev.value
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

    local root = recParse()
    if token then
        error {
            msg = "Extra closing parenthesis",
            loc = token.loc
        }
    end
    return root
end

return parse
