/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { Brand } from "./Brand";

/**
 * Creates a fake brand
 *
 * @returns A fake brand
 */
export const fakeBrand = (): Brand => new Brand(faker.company.name());
