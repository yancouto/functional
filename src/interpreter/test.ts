
import { tokenize, TokenType } from "./tokenizer";

function nullthrows<T>(x: T | void): T {
    if (x != null) {
        return x;
    } else {
        throw new Error("null!");
    }
}

function tokenizer_test() {
    const iter = tokenize("a: b (c:de)");
    assert(nullthrows(iter.next().value).value == "a");
    assert(nullthrows(iter.next().value).type == TokenType.SYMBOL);
    assert(nullthrows(iter.next().value).type == TokenType.TEXT);
    assert(nullthrows(iter.next().value).value == "(");
    assert(nullthrows(iter.next().value).value == "c");
    assert(nullthrows(iter.next().value).value == ":");
    assert(nullthrows(iter.next().value).value == "de");
    assert(nullthrows(iter.next().value).type == TokenType.SYMBOL);
    assert(iter.next().done);
}

const tests = [tokenizer_test];
const failed = tests.map(test => pcall(test)).filter(result => result[0] == false).pop();
if (failed == null) {
    print("All tests passed! :)")
} else {
    print("Some tests failed :(");
    print(failed[1]);
    os.exit(1);
}

