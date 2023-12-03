import { PlainDate } from "../valueObjects/PlainDate";
import Quantity from "../valueObjects/Quantity";
import { UUID } from "../valueObjects/UUID";

export type StashItem = {
    id: UUID;
    quantity: Quantity;
    expiryDate: PlainDate;
};
