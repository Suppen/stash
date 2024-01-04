import { Product } from "../domain/entities/Product";
import { StashItem } from "../domain/entities/StashItem";
import PlainDate from "../domain/valueObjects/PlainDate";

export type ProductService = {
    /**
     * Gets all products with at least one stash item
     *
     * @returns All products with at least one stash item
     *
     * @throws Whatever the implementation throws
     */
    getAllProductsWithStashItems: () => Promise<Product[]>;

    /**
     * Gets a single product by its ID
     *
     * @param productId ID of the product to get
     *
     * @returns The product with the given ID, or null if no product with that ID exists
     *
     * @throws Whatever the implementation throws
     */
    getProduct: (productId: Product["id"]) => Promise<Product | null>;

    /**
     * Creates a new product
     *
     * @param product Product to create
     *
     * @returns The created product
     *
     * @throws If the product already exists
     * @throws Whatever the implementation throws
     */
    createProduct: (product: Product) => Promise<Product>;

    /**
     * Updates an existing product
     *
     * @param product Product to update
     *
     * @returns The updated product
     *
     * @throws If the product does not exist
     * @throws Whatever the implementation throws
     */
    updateProduct: (product: Product) => Promise<Product>;

    /**
     * Deletes a product
     *
     * @param productId ID of the product to delete
     *
     * @throws If the product does not exist
     * @throws Whatever the implementation throws
     */
    deleteProduct: (productId: Product["id"]) => Promise<void>;

    /**
     * Adds a stash item to a product
     *
     * @param productId ID of the product to add a stash item to
     * @param stashItem Stash item to add
     *
     * @throws If the product does not exist
     * @throws If the stash item already exists
     * @throws Whatever the implementation throws
     */
    addStashItem: (productId: Product["id"], stashItem: StashItem) => Promise<void>;

    /**
     * Updates a stash item
     *
     * @param productId ID of the product that contains the stash item
     * @param stashItem Stash item to update
     *
     * @throws If the product does not exist
     * @throws If the stash item does not exist
     * @throws Whatever the implementation throws
     */
    updateStashItem: (productId: Product["id"], stashItem: StashItem) => Promise<StashItem>;

    /**
     * Deletes a stash item
     *
     * @param productId ID of the product that contains the stash item
     * @param stashItemId ID of the stash item to delete
     *
     * @throws If the product does not exist
     * @throws If the stash item does not exist
     * @throws Whatever the implementation throws
     */
    deleteStashItem: (productId: Product["id"], stashItemId: StashItem["id"]) => Promise<void>;

    /**
     * Gets a product by the ID of a stash item belonging to it
     *
     * @param stashItemId ID of the stash item
     *
     * @returns The product that contains the stash item, or null if no product contains the stash item
     *
     * @throws Whatever the implementation throws
     */
    getProductByStashItemId: (stashItemId: StashItem["id"]) => Promise<Product | null>;

    /**
     * Gets all products that expire before a given date, excluding the given date
     *
     * @param date Date to compare to
     *
     * @returns All products that expire before the given date, excluding the given date
     *
     * @throws Whatever the implementation throws
     */
    getProductsExpiringBefore: (date: PlainDate) => Promise<Product[]>;
};
