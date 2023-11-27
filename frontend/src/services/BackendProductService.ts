/* eslint-disable class-methods-use-this */
/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/require-await */
import { Product } from "../domain/entities/Product";
import { StashItem } from "../domain/entities/StashItem";
import PlainDate from "../domain/valueObjects/PlainDate";
import { createJSONFetcher } from "../utils/createJSONFetcher";
import { fromProduct, productDTOSchema, toProduct } from "./ProductDTO";
import { ProductService } from "./ProductService";
import { fromStashItem, stashItemDTOSchema, toStashItem } from "./StashItemDTO";

export class BackendProductService implements ProductService {
    /** The fetcher to use for requests */
    #fetcher: typeof fetch;

    /** Base URL to use for requests */
    #baseUrl: string;

    constructor({ fetcher, baseUrl }: { fetcher?: typeof fetch; baseUrl: string }) {
        this.#fetcher = fetcher ?? fetch.bind(window);
        this.#baseUrl = baseUrl;
    }

    async getProduct(productId: Product["id"]): Promise<Product | null> {
        try {
            return await createJSONFetcher(
                data => toProduct(productDTOSchema.parse(data)),
                this.#fetcher
            )(`${this.#baseUrl}/products/${productId.value()}`);
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 404) {
                    return null;
                }
            }
            throw err;
        }
    }

    async createProduct(product: Product): Promise<Product> {
        try {
            return await createJSONFetcher(data => toProduct(productDTOSchema.parse(data)), this.#fetcher)(
                `${this.#baseUrl}/products`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(fromProduct(product))
                }
            );
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 409) {
                    throw new Error("Product already exists");
                }
            }
            throw err;
        }
    }

    async updateProduct(product: Product): Promise<Product> {
        try {
            return await createJSONFetcher(data => toProduct(productDTOSchema.parse(data)), this.#fetcher)(
                `${this.#baseUrl}/products/${product.id.toString()}`,
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(fromProduct(product))
                }
            );
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 404) {
                    throw new Error("Product does not exist");
                }
            }
            throw err;
        }
    }

    async deleteProduct(productId: Product["id"]): Promise<void> {
        try {
            await this.#fetcher(`${this.#baseUrl}/products/${productId.toString()}`, {
                method: "DELETE"
            });
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 404) {
                    throw new Error("Product does not exist");
                }
            }
            throw err;
        }
    }

    async addStashItem(productId: Product["id"], stashItem: StashItem): Promise<void> {
        try {
            await this.#fetcher(`${this.#baseUrl}/products/${productId.toString()}/stash_items`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(fromStashItem(stashItem))
            });
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 404) {
                    throw new Error("Product does not exist");
                }
                if (err.status === 409) {
                    throw new Error("Stash item already exists");
                }
            }
            throw err;
        }
    }

    async updateStashItem(productId: Product["id"], stashItem: StashItem): Promise<StashItem> {
        try {
            return await createJSONFetcher(data => toStashItem(stashItemDTOSchema.parse(data)), this.#fetcher)(
                `${this.#baseUrl}/products/${productId.toString()}/stash_items/${stashItem.id}`,
                {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(fromStashItem(stashItem))
                }
            );
        } catch (err) {
            if (err instanceof Response) {
                const text = await err.text();
                if (err.status === 404) {
                    throw new Error(text);
                }
                if (err.status === 409) {
                    throw new Error(text);
                }
            }
            throw err;
        }
    }

    async deleteStashItem(productId: Product["id"], stashItemId: StashItem["id"]): Promise<void> {
        try {
            await this.#fetcher(`${this.#baseUrl}/products/${productId.toString()}/stash_items/${stashItemId}`, {
                method: "DELETE"
            });
        } catch (err) {
            if (err instanceof Response) {
                const text = await err.text();
                if (err.status === 404) {
                    throw new Error(text);
                }
            }
            throw err;
        }
    }

    async getProductByStashItemId(stashItemId: StashItem["id"]): Promise<Product | null> {
        try {
            return await createJSONFetcher(
                data => toProduct(productDTOSchema.parse(data)),
                this.#fetcher
            )(`${this.#baseUrl}/products/by_stash_item_id/${stashItemId}`);
        } catch (err) {
            if (err instanceof Response) {
                if (err.status === 404) {
                    return null;
                }
            }
            throw err;
        }
    }

    async getProductsExpiringBefore(date: PlainDate): Promise<Product[]> {
        return createJSONFetcher(
            data => productDTOSchema.array().parse(data).map(toProduct),
            this.#fetcher
        )(`${this.#baseUrl}/products/expiring_before/${date.toISOString()}`);

        // There is no known way for this to error
    }
}

export default BackendProductService;
