export function unifyString(name: string): string {
    if (name.includes("-")) {
        return name
            .split("-")
            .map((it) => unifyString(it))
            .join("");
    }
    return name.charAt(0).toUpperCase() + name.slice(1);
}
