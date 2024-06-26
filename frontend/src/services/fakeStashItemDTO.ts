/* c8 ignore start */
import { faker } from "@faker-js/faker";
import { StashItemDTO } from "./StashItemDTO";
import { fakeQuantity } from "../domain/valueObjects/fakeQuantity";
import PlainDate from "../domain/valueObjects/PlainDate";
import { UUID } from "../domain/valueObjects/UUID";

/**
 * Creates a fake stash item DTO
 *
 * @param nonFakedFields Fields that should not be faked
 *
 * @returns A fake stash item DTO
 */
export const fakeStashItemDTO = (nonFakedFields: Partial<StashItemDTO> = {}): StashItemDTO => ({
    id: UUID.v4().toString(),
    quantity: fakeQuantity().valueOf(),
    expiry_date: PlainDate.fromDate(faker.date.future()).toString(),
    ...nonFakedFields
});
