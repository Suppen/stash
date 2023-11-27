import { it, expect, describe, vi } from "vitest";
import { createJSONFetcher } from "./createJSONFetcher";

describe("the returned function", () => {
    it("should call the fetcher with the arguments the function is called with", async () => {
        const fetcher = vi.fn().mockResolvedValue({ json: () => Promise.resolve({}) });
        const url = "url";
        const options = { method: "POST" };

        const jsonFetcher = createJSONFetcher(vi.fn(), fetcher);

        await jsonFetcher(url, options);
        expect(fetcher).toHaveBeenCalledWith(url, options);
    });

    it("should call the parser with the parsed JSON data", async () => {
        const parser = vi.fn().mockReturnValue({});
        const data = { cake: "Cheese" };
        const fetcher = vi.fn().mockResolvedValue({ json: () => Promise.resolve(JSON.stringify(data)) });

        const jsonFetcher = createJSONFetcher(parser, fetcher);

        await jsonFetcher();
        expect(parser).toHaveBeenCalledWith(JSON.stringify(data));
    });

    it("should return what the parser returns", async () => {
        const expected = {};
        const parser = vi.fn<[], Record<string, unknown>>().mockReturnValue(expected);
        const fetcher = vi.fn().mockResolvedValue({ json: () => Promise.resolve({}) });

        const jsonFetcher = createJSONFetcher(parser, fetcher);

        const actual = await jsonFetcher();
        expect(actual).toBe(expected);
    });
});
