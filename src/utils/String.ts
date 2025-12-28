export function unifyString(name: string): string {
    if (name.includes("-")) {
        return name
            .split("-")
            .map((it) => unifyString(it))
            .join("");
    }
    return name.charAt(0).toUpperCase() + name.slice(1);
}

export function insertEvery(input: string, separator: string, interval: number = 4): string {
    let result = "";
    for (let i = 0; i < input.length; i++) {
        if (i > 0 && i % interval === 0) {
            result += separator;
        }
        result += input[i];
    }
    return result;
}
