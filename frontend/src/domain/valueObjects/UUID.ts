import { v4 as uuid, validate as validateUUID } from "uuid";

/** Guaranteed valid UUID */
export class UUID {
    #value: ReturnType<typeof uuid>;

    constructor(value: string) {
        if (!validateUUID(value)) {
            throw new Error("Invalid UUID");
        }

        this.#value = value;
    }

    static fromString(value: string) {
        return new UUID(value);
    }

    static v4() {
        return new UUID(uuid());
    }

    toString() {
        return this.#value;
    }

    value() {
        return this.#value;
    }
}
