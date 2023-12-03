/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { StashItem } from "./StashItem";
import { fakeQuantity } from "../valueObjects/fakeQuantity";
import PlainDate from "../valueObjects/PlainDate";
import { UUID } from "../valueObjects/UUID";

/**
 * Creates a fake stash item
 *
 * @param nonFakedFields Fields that should not be faked
 *
 * @returns A fake stash item
 */
export const fakeStashItem = (nonFakedFields: Partial<StashItem> = {}): StashItem => ({
    id: UUID.v4(),
    quantity: fakeQuantity(),
    expiryDate: PlainDate.fromDate(faker.date.future()),
    ...nonFakedFields
});
