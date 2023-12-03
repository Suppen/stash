import { describe, it, expect } from "vitest";
import { UUID } from "./UUID";

describe("Construction", () => {
    it("should succeed on a valid UUID string", () => {
        expect(() => new UUID("00000000-0000-0000-0000-000000000000")).not.toThrow();
    });

    it("should throw on a string not on the format XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX", () => {
        expect(() => new UUID("00000000-0000-0000-0000-00000000000")).toThrow();
    });

    it("should throw on an invalid UUID string", () => {
        expect(() => new UUID("00000000-0000-0000-0000-00000000000g")).toThrow();
    });
});

describe("fromString", () => {
    it("should create a UUID from a string", () => {
        const uuidStr = "00000000-0000-0000-0000-000000000000";
        const uuid = UUID.fromString(uuidStr);

        expect(uuid.toString()).toBe(uuidStr);
    });
});

describe("v4", () => {
    it("should create a random UUID", () => {
        const uuid = UUID.v4();

        expect(uuid.toString()).toMatch(/^[a-f0-9]{8}-([a-f0-9]{4}-){3}[a-f0-9]{12}$/);
    });
});

describe("toString", () => {
    it("should return the UUID string", () => {
        const uuidStr = "00000000-0000-0000-0000-000000000000";
        const uuid = new UUID(uuidStr);

        expect(uuid.toString()).toBe(uuidStr);
    });
});

describe("value", () => {
    it("should return the UUID string", () => {
        const uuidStr = "00000000-0000-0000-0000-000000000000";
        const uuid = new UUID(uuidStr);

        expect(uuid.value()).toBe(uuidStr);
    });
});
