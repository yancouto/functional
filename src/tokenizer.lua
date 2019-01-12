local function tokenizeCoroutine(str)
	local i = 1
	for j = 1, #str do
		local c = str:sub(j, j)
		if not c:match("%a") then
			if i < j then
				coroutine.yield {
					value = str:sub(i, j - 1),
					loc = {i, j - 1}
				}
			end
			if not c:match("%s") then
				coroutine.yield {
					value = c,
					loc = {j, j}
				}
			end
			i = j + 1
		end
	end
	if i <= #str then
		coroutine.yield {
			value = str:sub(i),
			loc = {i, #str}
		}
	end
end

local function tokenize(str)
	local i = str:find("[^%a%s():]")
	if i then
		error {
			msg = "Invalid character " .. str:sub(i, i),
			loc = {i, i}
		}
	end
	return coroutine.wrap(function() tokenizeCoroutine(str) end)
end

return tokenize
