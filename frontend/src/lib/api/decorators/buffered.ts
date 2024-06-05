export function buffered<A extends any[], R extends object>(
    fn: (input: Record<string, A>) => Promise<Record<string, R>>,
    interval: number = 1000
): (...args: A) => Promise<R> {
    type Executor = [(value: R) => void, (error: any) => void]
    const buffer = new Array<[A, Executor]>()

    async function consumeBuffer() {
        let inputs: Record<string, A> = {}
        let executors: Record<string, Executor> = {}
        for (const [index, [input, executor]] of buffer.entries()) {
            inputs[index] = input
            executors[index] = executor
        }

        buffer.length = 0
        let outputs = await fn(inputs)

        for (const [index, [resolve, reject]] of Object.entries(executors)) {
            let output = outputs[index]
            if (output === undefined) {
                reject(`no value for id '${index}'`)
                continue
            }
            resolve(output)
        }
    }

    return function (...args: A) {
        return new Promise((resolve, reject) => {
            buffer.push([args, [resolve, reject]])
            if (buffer.length <= 1) {
                setTimeout(() => consumeBuffer(), interval)
            }
        })
    }
}
