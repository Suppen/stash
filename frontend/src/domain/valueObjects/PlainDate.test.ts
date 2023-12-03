import { describe, it, expect } from "vitest";
import { PlainDate } from "./PlainDate";

describe("Construction", () => {
    it("should succeed on a valid date string", () => {
        expect(() => new PlainDate("2021-01-01")).not.toThrow();
    });

    it("should throw on a string not on the format YYYY-MM-DD", () => {
        expect(() => new PlainDate("2021-1-1")).toThrow();
    });

    it("should throw on an invalid date string", () => {
        expect(() => new PlainDate("2021-02-31")).toThrow();
    });
});

describe("fromDate", () => {
    it("should create a PlainDate from a Date object", () => {
        const date = new Date("2021-01-01");
        const plainDate = PlainDate.fromDate(date);

        expect(plainDate.toString()).toBe("2021-01-01");
    });
});

describe("toString", () => {
    it("should return the date string", () => {
        const dateStr = "2021-01-01";
        const date = new PlainDate(dateStr);

        expect(date.toString()).toBe(dateStr);
    });
});

describe("toISOString", () => {
    it("should return the date string", () => {
        const dateStr = "2021-01-01";
        const date = new PlainDate(dateStr);

        expect(date.toISOString()).toBe(dateStr);
    });
});
