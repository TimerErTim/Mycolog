const DEF_DELAY = 1000;

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, Number.isNaN(Number(ms)) ? DEF_DELAY : Number(ms)));
}