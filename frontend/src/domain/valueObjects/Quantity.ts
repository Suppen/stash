/** Quantity of a stash item. Guaranteed to be a positive integer */
export class Quantity {
    /** Quantity of a stash item. Guaranteed to be a positive integer */
    #value: number;

    constructor(value: number) {
        // Make sure the quantity is positive
        if (value < 0) {
            throw new Error("Quantity cannot be negative");
        }

        // Make sure the quantity is an integer
        if (!Number.isInteger(value)) {
            throw new Error("Quantity must be an integer");
        }

        this.#value = value;
    }

    /** Returns the quantity */
    value() {
        return this.#value;
    }

    /** Returns the quantity */
    valueOf() {
        return this.#value;
    }
}

export default Quantity;
