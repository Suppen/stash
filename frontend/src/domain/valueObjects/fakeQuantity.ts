/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { Quantity } from "./Quantity";

/**
 * Creates a fake quantity
 *
 * @returns A fake quantity
 */
export const fakeQuantity = (): Quantity => new Quantity(faker.number.int({ min: 1, max: 100 }));
