import { describe, it, expect } from "vitest";
import { Brand } from "./Brand";

describe("Construction", () => {
    it("should succeed on a non-empty string", () => {
        expect(() => new Brand("1")).not.toThrow();
    });

    it("should throw on an empty string", () => {
        expect(() => new Brand("")).toThrow();
    });
});

describe("value", () => {
    it("should return the value", () => {
        const value = "1";
        const brand = new Brand(value);

        expect(brand.value()).toBe(value);
    });
});

describe("toString", () => {
    it("should return the value", () => {
        const value = "1";
        const brand = new Brand(value);

        expect(brand.toString()).toBe(value);
    });
});
