import type { Uuid } from "./apis/apelle";

export function isUuid(param: string): param is Uuid {
    return /^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$/.test(param);
}