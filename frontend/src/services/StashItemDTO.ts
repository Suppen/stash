import * as z from "zod";
import Quantity from "../domain/valueObjects/Quantity";
import PlainDate from "../domain/valueObjects/PlainDate";
import { StashItem } from "../domain/entities/StashItem";
import { UUID } from "../domain/valueObjects/UUID";

export const stashItemDTOSchema = z.object({
    id: z.string().uuid(),
    quantity: z.number(),
    expiry_date: z.string()
});

export type StashItemDTO = z.infer<typeof stashItemDTOSchema>;

export const fromStashItem = (stashItem: StashItem): StashItemDTO => ({
    id: stashItem.id.toString(),
    quantity: stashItem.quantity.value(),
    expiry_date: stashItem.expiryDate.toISOString()
});

export const toStashItem = (stashItemDTO: StashItemDTO): StashItem => ({
    id: UUID.fromString(stashItemDTO.id),
    quantity: new Quantity(stashItemDTO.quantity),
    expiryDate: new PlainDate(stashItemDTO.expiry_date)
});
