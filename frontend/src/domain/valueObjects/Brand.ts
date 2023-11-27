/** Brand of a product */
export class Brand {
    #value: string;

    constructor(value: string) {
        if (!value) {
            throw new Error("Brand cannot be empty");
        }

        this.#value = value;
    }

    value() {
        return this.#value;
    }

    toString() {
        return this.#value;
    }
}

export default Brand;
