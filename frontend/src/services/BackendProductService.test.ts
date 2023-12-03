import { describe, it, expect, beforeEach, vi } from "vitest";
import { BackendProductService } from "./BackendProductService";
import ProductId from "../domain/valueObjects/ProductId";
import { fakeProductDTO } from "./fakeProductDTO";
import { fromProduct, toProduct } from "./ProductDTO";
import { fakeProduct } from "../domain/entities/fakeProduct";
import { fakeProductId } from "../domain/valueObjects/fakeProductId";
import { fakeStashItem } from "../domain/entities/fakeStashItem";
import { fromStashItem } from "./StashItemDTO";
import PlainDate from "../domain/valueObjects/PlainDate";

const baseUrl = "http://fakebackend.com";
const fetcher = vi.fn<Parameters<typeof fetch>, ReturnType<typeof fetch>>();
let productService: BackendProductService;
beforeEach(() => {
    fetcher.mockReset();
    productService = new BackendProductService({ fetcher: fetcher as typeof fetch, baseUrl });
});

describe("getProduct", () => {
    it("should call the correct URL with the correct data", async () => {
        const productId = new ProductId("ID");
        fetcher.mockResolvedValueOnce(Response.json(fakeProductDTO({ id: productId.toString() })));

        await productService.getProduct(productId);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/${productId.value()}`);
    });

    it("should return a product", async () => {
        const productId = new ProductId("ID");
        const productDTO = fakeProductDTO({ id: productId.toString() });
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        const product = await productService.getProduct(productId);
        expect(product).toEqual(toProduct(productDTO));
    });

    it("should return null if the response code is 404", async () => {
        const productId = new ProductId("ID");
        fetcher.mockResolvedValueOnce(new Response(null, { status: 404 }));

        const product = await productService.getProduct(productId);
        expect(product).toBeNull();
    });
});

describe("createProduct", () => {
    it("should call the correct URL with the correct data", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        await productService.createProduct(product);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(productDTO)
        });
    });

    it("should return the created product", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        const result = await productService.createProduct(product);
        expect(result).toEqual(product);
    });

    it("should throw if the response code is 409", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO, { status: 409 }));

        await expect(productService.createProduct(product)).rejects.toThrow();
    });
});

describe("updateProduct", () => {
    it("should call the correct URL with the correct data", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        await productService.updateProduct(product);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/${product.id.toString()}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(productDTO)
        });
    });

    it("should return the updated product", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        const result = await productService.updateProduct(product);
        expect(result).toEqual(product);
    });

    it("should throw if the response code is 404", async () => {
        const product = fakeProduct();
        fetcher.mockResolvedValueOnce(Response.json(null, { status: 404 }));

        await expect(productService.updateProduct(product)).rejects.toThrow();
    });
});

describe("deleteProduct", () => {
    it("should call the correct URL with the correct data", async () => {
        const productId = fakeProductId();
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await productService.deleteProduct(productId);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/${productId.value()}`, {
            method: "DELETE"
        });
    });

    it("should not return anything", async () => {
        const productId = fakeProductId();
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await expect(productService.deleteProduct(productId)).resolves.toBeUndefined();
    });
});

describe("addStashItem", () => {
    it("should call the correct URL with the correct data", async () => {
        const productId = fakeProductId();
        const stashItem = fakeStashItem();
        const stashItemDTO = fromStashItem(stashItem);
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await productService.addStashItem(productId, stashItem);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/${productId.value()}/stash_items`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(stashItemDTO)
        });
    });

    it("should not return anything", async () => {
        const productId = fakeProductId();
        const stashItem = fakeStashItem();
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await expect(productService.addStashItem(productId, stashItem)).resolves.toBeUndefined();
    });

    it("should throw if the response code is 404", async () => {
        fetcher.mockResolvedValueOnce(new Response(null, { status: 404 }));

        await expect(productService.addStashItem(fakeProductId(), fakeStashItem())).rejects.toThrow();
    });

    it("should throw if the response code is 409", async () => {
        fetcher.mockResolvedValueOnce(new Response(null, { status: 409 }));

        await expect(productService.addStashItem(fakeProductId(), fakeStashItem())).rejects.toThrow();
    });
});

describe("updateStashItem", () => {
    it("should call the correct URL with the correct data", async () => {
        const productId = fakeProductId();
        const stashItem = fakeStashItem();
        const stashItemDTO = fromStashItem(stashItem);
        fetcher.mockResolvedValueOnce(Response.json(stashItemDTO));

        await productService.updateStashItem(productId, stashItem);
        expect(fetcher).toHaveBeenCalledWith(
            `${baseUrl}/products/${productId.value()}/stash_items/${stashItem.id.toString()}`,
            {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(stashItemDTO)
            }
        );
    });

    it("should return the updated stash item", async () => {
        const productId = fakeProductId();
        const stashItem = fakeStashItem();
        const stashItemDTO = fromStashItem(stashItem);
        fetcher.mockResolvedValueOnce(Response.json(stashItemDTO));

        const result = await productService.updateStashItem(productId, stashItem);
        expect(result).toEqual(stashItem);
    });

    it("should throw if the response code is 404", async () => {
        fetcher.mockResolvedValueOnce(new Response(null, { status: 404 }));

        await expect(productService.updateStashItem(fakeProductId(), fakeStashItem())).rejects.toThrow();
    });
});

describe("deleteStashItem", () => {
    it("should call the correct URL with the correct data", async () => {
        const productId = fakeProductId();
        const stashItemId = fakeStashItem().id;
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await productService.deleteStashItem(productId, stashItemId);
        expect(fetcher).toHaveBeenCalledWith(
            `${baseUrl}/products/${productId.value()}/stash_items/${stashItemId.toString()}`,
            {
                method: "DELETE"
            }
        );
    });

    it("should not return anything", async () => {
        const productId = fakeProductId();
        const stashItemId = fakeStashItem().id;
        fetcher.mockResolvedValueOnce(new Response(null, { status: 204 }));

        await expect(productService.deleteStashItem(productId, stashItemId)).resolves.toBeUndefined();
    });

    it("should throw if the response code is 404", async () => {
        fetcher.mockResolvedValueOnce(new Response(null, { status: 404 }));

        await expect(productService.deleteStashItem(fakeProductId(), fakeStashItem().id)).rejects.toThrow();
    });
});

describe("getProductByStashItemId", () => {
    it("should call the correct URL with the correct data", async () => {
        const stashItemId = fakeStashItem().id;
        fetcher.mockResolvedValueOnce(Response.json(fakeProductDTO()));

        await productService.getProductByStashItemId(stashItemId);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/by_stash_item_id/${stashItemId.toString()}`);
    });

    it("should return the product", async () => {
        const product = fakeProduct();
        const productDTO = fromProduct(product);
        fetcher.mockResolvedValueOnce(Response.json(productDTO));

        const result = await productService.getProductByStashItemId(fakeStashItem().id);
        expect(result).toEqual(product);
    });

    it("should return null if the response code is 404", async () => {
        fetcher.mockResolvedValueOnce(new Response(null, { status: 404 }));

        const result = await productService.getProductByStashItemId(fakeStashItem().id);
        expect(result).toBeNull();
    });
});

describe("getProductsExpiringBefore", () => {
    it("should call the correct URL with the correct data", async () => {
        const date = PlainDate.fromDate(new Date());
        fetcher.mockResolvedValueOnce(Response.json([fakeProductDTO()]));

        await productService.getProductsExpiringBefore(date);
        expect(fetcher).toHaveBeenCalledWith(`${baseUrl}/products/expiring_before/${date.toISOString()}`);
    });

    it("should return the products", async () => {
        const products = [fakeProduct()];
        const productDTOs = products.map(fromProduct);
        fetcher.mockResolvedValueOnce(Response.json(productDTOs));

        const result = await productService.getProductsExpiringBefore(PlainDate.fromDate(new Date()));
        expect(result).toEqual(products);
    });
});
