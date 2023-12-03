/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { ProductDTO } from "./ProductDTO";
import { fakeProductId } from "../domain/valueObjects/fakeProductId";
import { fakeBrand } from "../domain/valueObjects/fakeBrand";
import { fakeStashItemDTO } from "./fakeStashItemDTO";

/**
 * Creates a fake product DTO
 *
 * @param nonFakedFields Fields that should not be faked
 *
 * @returns A fake product DTO
 */
export const fakeProductDTO = (nonFakedFields: Partial<ProductDTO> = {}): ProductDTO => ({
    id: fakeProductId().toString(),
    brand: fakeBrand().toString(),
    name: faker.commerce.productName(),
    stash_items: Array.from({ length: faker.number.int({ min: 0, max: 2 }) }, () => fakeStashItemDTO()),
    ...nonFakedFields
});
