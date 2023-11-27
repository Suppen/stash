import { describe, it, expect } from "vitest";
import { Quantity } from "./Quantity";

describe("Construction", () => {
    it("should succeed on a positive integer", () => {
        expect(() => new Quantity(1)).not.toThrow();
    });

    it("should throw on a negative integer", () => {
        expect(() => new Quantity(-1)).toThrow();
    });

    it("should throw on a non-integer", () => {
        expect(() => new Quantity(1.5)).toThrow();
    });
});

describe("value", () => {
    it("should return the value", () => {
        const value = 1;
        const quantity = new Quantity(value);

        expect(quantity.value()).toBe(value);
    });
});

describe("valueOf", () => {
    it("should return the value", () => {
        const value = 1;
        const quantity = new Quantity(value);

        expect(quantity.valueOf()).toBe(value);
    });
});
