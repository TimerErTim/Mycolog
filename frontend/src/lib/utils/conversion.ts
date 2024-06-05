export function objectToRecord(obj: object) {
    return Object.entries(obj).reduce((acc, [key, value]) => {
        acc[key] = JSON.stringify(value)
        return acc
    }, {} as Record<string, string>)
}

export function mapRecordValues<I, O>(record: Record<string, I>, fn: (value: I) => O) {
    return Object.entries(record).reduce((acc, [key, value]) => {
        acc[key] = fn(value)
        return acc
    }, {} as Record<string, O>)
}
