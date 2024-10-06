import assert from "assert";

let _argv: string[] = [];

export function argv(): string {
    if ( _argv.length == 0 ) _argv = process.argv.slice(2);

    const arg = _argv.shift();
    assert(arg, "Required argv not present");
    
    return arg;
}