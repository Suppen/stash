/**
 * Creates a function that takes the same arguments as `fetch`, and returns a promise with JSON data parsed to the
 * wanted type
 *
 * @param parser - Function that parses the parsed JSON data to the wanted type
 *
 * @returns Function that takes the same arguments as `fetch`, and returns a promise with JSON data parsed to the
 * wanted type
 */
export function createJSONFetcher<T, FP extends Parameters<typeof fetch>>(
    parser: (data: unknown) => T,
    fetcher: (...args: FP) => Promise<Response> = fetch
) {
    return async (...args: FP): Promise<T> => {
        const response = await fetcher(...args);
        const data = (await response.json()) as unknown;
        return parser(data);
    };
}
