local parse = require "parser"
local gen = require "generator"
local interpret = require "interpreter"

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

local tests = {
    { "(x: x x x) (y: y) oi", "oi" },
    { "(z: (x : z : x) z) aaa", "z: aaa" },
    { "(a:b: b a) aaa bbb", "bbb aaa" },
    { "(a:b:c: c a b) aaa bbb (a: b: a)", "aaa" },
    { "(a:b:c: c a b) aaa bbb (a: b: b)", "bbb" },
}

local all_ok = true

for i, t in ipairs(tests) do
    local ok, ans = pcall(exhaust, function() return interpret(t[1]) end)
    local ok2, parsed_correct = pcall(parse, t[2])
    if not ok then
        io.write("Test #", i, ' "', t[1], '" failed: ERROR!\n')
        io.write("  ", type(ans) == 'string' and ans or ans.msg, "\n\n")
    elseif not ok2 then
        io.write("Test #", i, ' "', t[1], '" failed: ERROR!\n')
        io.write("  ", parsed_correct.msg, "\n\n")
    elseif not deepEquals(ans, parsed_correct) then
        io.write("Test #", i, ' "', t[1], '" failed:\n')
        io.write("  Expected: ", t[2], "\n")
        io.write("  Got: ", gen(ans), "\n\n")
        all_ok = false
    end
end

if all_ok then
    print("All tests passed!")
else
    print("Some tests failed :(")
    os.exit(1)
end
