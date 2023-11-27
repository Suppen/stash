import { describe, it, expect } from "vitest";
import { ProductId } from "./ProductId";

describe("Construction", () => {
    it("should succeed on a non-empty string", () => {
        expect(() => new ProductId("1")).not.toThrow();
    });

    it("should throw on an empty string", () => {
        expect(() => new ProductId("")).toThrow();
    });
});

describe("value", () => {
    it("should return the value", () => {
        const value = "1";
        const productId = new ProductId(value);

        expect(productId.value()).toBe(value);
    });
});

describe("toString", () => {
    it("should return the value", () => {
        const value = "1";
        const productId = new ProductId(value);

        expect(productId.toString()).toBe(value);
    });
});
