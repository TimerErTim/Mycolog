const emailRegex: RegExp = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

export function isValidEmail(email: string): boolean {
    return emailRegex.test(email)
}
