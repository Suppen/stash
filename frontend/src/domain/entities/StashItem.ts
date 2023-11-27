import { PlainDate } from "../valueObjects/PlainDate";
import Quantity from "../valueObjects/Quantity";

export type StashItem = {
    id: ReturnType<typeof crypto.randomUUID>;
    quantity: Quantity;
    expiryDate: PlainDate;
};
