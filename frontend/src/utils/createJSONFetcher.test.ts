import { it, expect, describe, vi } from "vitest";
import { createJSONFetcher } from "./createJSONFetcher";

describe("the returned function", () => {
    it("should call the fetcher with the arguments the function is called with", async () => {
        const fetcher = vi.fn().mockResolvedValue(Response.json({}));
        const url = "url";
        const options = { method: "POST" };

        const jsonFetcher = createJSONFetcher(vi.fn(), fetcher);

        await jsonFetcher(url, options);
        expect(fetcher).toHaveBeenCalledWith(url, options);
    });

    it("should call the parser with the parsed JSON data", async () => {
        const parser = vi.fn().mockReturnValue({});
        const data = { cake: "Cheese" };
        const fetcher = vi.fn().mockResolvedValue(Response.json(data));

        const jsonFetcher = createJSONFetcher(parser, fetcher);

        await jsonFetcher();
        expect(parser).toHaveBeenCalledWith(data);
    });

    it("should return what the parser returns", async () => {
        const expected = {};
        const parser = vi.fn<[], Record<string, unknown>>().mockReturnValue(expected);
        const fetcher = vi.fn().mockResolvedValue(Response.json({}));

        const jsonFetcher = createJSONFetcher(parser, fetcher);

        const actual = await jsonFetcher();
        expect(actual).toBe(expected);
    });

    it("should throw a network error if the request fails", async () => {
        const fetcher = vi.fn().mockRejectedValue(new Error("Network error"));

        const jsonFetcher = createJSONFetcher(vi.fn(), fetcher);

        await expect(jsonFetcher()).rejects.toThrow("Network error");
    });

    it("should throw the response if the response is not ok", async () => {
        const response = Response.error();
        const fetcher = vi.fn().mockResolvedValue(response);

        const jsonFetcher = createJSONFetcher(vi.fn(), fetcher);

        await expect(jsonFetcher()).rejects.toBe(response);
    });

    it("should throw a JSON parse error if the response is not valid JSON", async () => {
        const fetcher = vi.fn().mockResolvedValue({
            ok: true,
            json: () => Promise.resolve().then(() => JSON.parse("Not JSON") as unknown)
        });

        const jsonFetcher = createJSONFetcher(vi.fn(), fetcher);

        await expect(jsonFetcher()).rejects.toThrow();
    });

    it("should throw whatever error the parser throws", async () => {
        const parser = vi.fn().mockImplementation(() => {
            throw new Error("Parser error");
        });
        const fetcher = vi.fn().mockResolvedValue(Response.json({}));

        const jsonFetcher = createJSONFetcher(parser, fetcher);

        await expect(jsonFetcher()).rejects.toThrow("Parser error");
    });
});
