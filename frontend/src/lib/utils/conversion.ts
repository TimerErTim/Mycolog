export function objectToRecord(obj: object) {
    return Object.entries(obj).reduce((acc, [key, value]) => {
        acc[key] = JSON.stringify(value)
        return acc
    }, {} as Record<string, string>)
}