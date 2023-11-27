import Brand from "../valueObjects/Brand";
import { ProductId } from "../valueObjects/ProductId";
import { StashItem } from "./StashItem";

export type Product = {
    id: ProductId;
    brand: Brand;
    name: string;
    stashItems: StashItem[];
};
