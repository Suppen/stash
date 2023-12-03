/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { ProductId } from "./ProductId";

/**
 * Creates a fake product ID
 *
 * @returns A fake product ID
 */
export const fakeProductId = (): ProductId => new ProductId(faker.string.alphanumeric(10));
