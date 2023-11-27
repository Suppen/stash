/** ID of a product */
export class ProductId {
    #value: string;

    constructor(value: string) {
        if (!value) {
            throw new Error("Product ID cannot be empty");
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

export default ProductId;
