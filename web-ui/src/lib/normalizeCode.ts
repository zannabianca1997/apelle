export default function normalizeCode(code: string | null): string | null {
    return code?.trim().toUpperCase() || null;
}