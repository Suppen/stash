import * as z from "zod";
import { Product } from "../domain/entities/Product";
import Brand from "../domain/valueObjects/Brand";
import ProductId from "../domain/valueObjects/ProductId";
import { fromStashItem, stashItemDTOSchema, toStashItem } from "./StashItemDTO";

export const productDTOSchema = z.object({
    id: z.string(),
    brand: z.string(),
    name: z.string(),
    stash_items: z.array(stashItemDTOSchema)
});

export type ProductDTO = z.infer<typeof productDTOSchema>;

export const fromProduct = (product: Product): ProductDTO => ({
    id: product.id.toString(),
    brand: product.brand.toString(),
    name: product.name,
    stash_items: product.stashItems.map(fromStashItem)
});

export const toProduct = (productDTO: ProductDTO): Product => ({
    id: new ProductId(productDTO.id),
    brand: new Brand(productDTO.brand),
    name: productDTO.name,
    stashItems: productDTO.stash_items.map(toStashItem)
});
