/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { Product } from "./Product";
import { fakeProductId } from "../valueObjects/fakeProductId";
import { fakeBrand } from "../valueObjects/fakeBrand";
import { fakeStashItem } from "./fakeStashItem";

/**
 * Creates a fake product
 *
 * @params nonFakedFields Fields that should not be faked
 *
 * @returns A fake product
 */
export const fakeProduct = (nonFakedFields: Partial<Product> = {}): Product => ({
    id: fakeProductId(),
    brand: fakeBrand(),
    name: faker.commerce.productName(),
    stashItems: Array.from({ length: faker.number.int({ min: 0, max: 2 }) }, () => fakeStashItem()),
    ...nonFakedFields
});
