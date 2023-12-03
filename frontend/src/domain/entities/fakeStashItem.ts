/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { StashItem } from "./StashItem";
import { fakeQuantity } from "../valueObjects/fakeQuantity";
import PlainDate from "../valueObjects/PlainDate";

/**
 * Creates a fake stash item
 *
 * @param nonFakedFields Fields that should not be faked
 *
 * @returns A fake stash item
 */
export const fakeStashItem = (nonFakedFields: Partial<StashItem> = {}): StashItem => ({
    id: faker.string.uuid() as StashItem["id"],
    quantity: fakeQuantity(),
    expiryDate: PlainDate.fromDate(faker.date.future()),
    ...nonFakedFields
});
