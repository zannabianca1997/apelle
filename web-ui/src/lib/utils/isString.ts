export default function isString(s: string | unknown): s is string {
  return typeof s == 'string' || s instanceof String;
}
