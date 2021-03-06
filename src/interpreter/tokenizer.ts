export enum TokenType {
    TEXT,
    SYMBOL,
};

type Token = { value: string, loc: [number, number], type: TokenType };

function* tokenizeImpl(str: string): Generator<Token, void, void> {
    let cur = [];
    let j = 0;
    for (let c of str) {
        j++;
        const is_letter = string.match(c, "%a") != null;
        const is_space = string.match(c, "%s");
        if (is_letter) {
            cur.push(c);
        } else {
            if (cur.length > 0) {
                yield {
                    value: cur.join(""),
                    loc: [j - cur.length, j - 1],
                    type: TokenType.TEXT
                };
                cur = [];
            }
            if (!is_space) {
                yield {
                    value: c,
                    loc: [j - 1, j - 1],
                    type: TokenType.SYMBOL
                }
            }
        }
    }
}

export function* tokenize(str: string): Generator<Token, void, void> {
    const result = string.find(str, "[^%a%s():]");
    // Can't to everything in one assignment because of a bug in tstl
    const [i, ...rest] = result;
    if (i != null) {
        throw { "msg": `Invalid character ${str.substr(i, i)}`, "loc": [i, i] };
    }
    yield* tokenizeImpl(str);
}