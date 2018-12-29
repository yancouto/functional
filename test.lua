local parse     = require "parser"
local gen       = require "generator"
local interpret = require "interpreter"
local utils     = require "utils"

local tests = {
    -- id
    { "id A", "A" },
    { "id B", "B" },
    { "id id A", "A" },
    -- swap
    { "(swap A B)", "B A" },
    { "swap A A", "A A" },
    { "swap A id B", "A B" },
    { "swap C swap D", "D C" },
    { "swap swap C D", "C swap D" },
    -- general
    { "(x: x x x) (y: y) A", "A" },
    { "(z: (x : z : x) z) A", "z: A" },
    { "(a:b: b a) A B", "B A" },
    { "(a:b:c: c a b) A B (a: b: a)", "A" },
    { "(a:b:c: c a b) A B (a: b: b)", "B" },
}

local all_ok = true

for i, t in ipairs(tests) do
    local ok, ans = pcall(utils.exhaust, function() return interpret(t[1], 1000) end)
    local ok2, parsed_correct = pcall(parse, t[2], 1000)
    if not ok then
        io.write("Test #", i, ' "', t[1], '" failed: ERROR!\n')
        io.write("  ", type(ans) == 'string' and ans or ans.msg, "\n\n")
    elseif not ok2 then
        io.write("Test #", i, ' "', t[1], '" failed: ERROR!\n')
        io.write("  ", parsed_correct.msg, "\n\n")
    elseif not utils.deepEquals(ans, parsed_correct) then
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
