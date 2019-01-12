local tokenizer        = require "tokenizer"
local deepCopyAndShift = require("utils").deepCopyAndShift

local constants = {
	id        = "x:x",
	swap      = "x:y: y x",
	['true']  = "x:y: x",
	['false'] = "x:y: y",
	['if']    = "c:a:b: c a b",
	['and']   = "a:b: a b false",
	['or']    = "a:b: a true b",
	pair      = "x:y:a: a x y",
	fst       = "true",
	snd       = "false",
}

local order = {"id", "swap", "true", "false", "if", "and", "or", "pair", "fst", "snd"}

local function parse(str, known_cts)
	known_cts = known_cts or 0
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
					-- single uppercase letter is not a constant
					elseif name ~= name:match("[A-Z]") then
						-- check if you're allowed to use that constant
						if not constants[name] or constants[name].pos > known_cts then
							error {
								msg = "Unknown constant '" .. name .. "'",
								loc = loc
							}
						end
						-- contants have no unbound variables
						-- so no shifting will happen
						node = deepCopyAndShift(constants[name].code, 0, 0)
					end
					node = node or {
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

-- initializing constants in correct order
for i, const_ in ipairs(order) do
	constants[const_] = {
		pos  = i,
		code = parse(constants[const_], i - 1)
	}
end

for const_, v in pairs(constants) do
	if type(v) == 'string' then
		error("Missing order for constant '" .. const_ .. "'")
	end
end


return parse
