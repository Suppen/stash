import { describe, it, expect, vi } from "vitest";
import { ProductForm } from "./ProductForm";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { fakeProduct } from "../../domain/entities/fakeProduct";
import { fakeStashItem } from "../../domain/entities/fakeStashItem";
import { RouterProvider, createMemoryRouter } from "react-router-dom";

const renderWithContext = (ui: Parameters<typeof render>[0], options?: Omit<Parameters<typeof render>[1], "wrapper">) =>
    render(ui, {
        ...options,
        wrapper: ({ children }) => (
            <RouterProvider
                router={createMemoryRouter([
                    {
                        path: "/",
                        element: children
                    }
                ])}
            />
        )
    });

describe("Default values", () => {
    it("should not have a default value for ID", () => {
        renderWithContext(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("id")).toHaveValue("");
    });

    it("should not have a default value for brand", () => {
        renderWithContext(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("brand")).toHaveValue("");
    });

    it("should not have a default value for name", () => {
        renderWithContext(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("name")).toHaveValue("");
    });

    it("should not have any stash items by default", () => {
        renderWithContext(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByText("noStashItems")).toBeInTheDocument();
    });
});

describe("Providing a product", () => {
    it("should have the provided ID", () => {
        const product = fakeProduct();
        renderWithContext(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("id")).toHaveValue(product.id.value());
    });

    it("should have the provided brand", () => {
        const product = fakeProduct();
        renderWithContext(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("brand")).toHaveValue(product.brand.value());
    });

    it("should have the provided name", () => {
        const product = fakeProduct();
        renderWithContext(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("name")).toHaveValue(product.name);
    });

    it("should have the provided stash items", () => {
        const product = fakeProduct({ stashItems: Array.from({ length: 3 }, () => fakeStashItem()) });
        const { container } = renderWithContext(<ProductForm onSubmit={vi.fn()} product={product} />);

        const expiryDateInputs = container.querySelectorAll("[name$=expiryDate]");
        expect(expiryDateInputs).toHaveLength(product.stashItems.length);
        expiryDateInputs.forEach((expiryDateInput, index) => {
            expect(expiryDateInput).toHaveValue(product.stashItems[index].expiryDate.toISOString());
        });

        const quantityInputs = document.querySelectorAll("[name$=quantity]");
        expect(quantityInputs).toHaveLength(product.stashItems.length);
        quantityInputs.forEach((quantityInput, index) => {
            expect(quantityInput).toHaveValue(product.stashItems[index].quantity.value());
        });
    });
});

describe("Providing a product ID", () => {
    it("should have the provided ID", () => {
        const productId = fakeProduct().id;
        renderWithContext(<ProductForm onSubmit={vi.fn()} productId={productId} />);

        expect(screen.getByLabelText("id")).toHaveValue(productId.value());
    });
});

describe("Validation", () => {
    it("requires an ID", async () => {
        const onSubmit = vi.fn();

        renderWithContext(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("idIsRequired")).toBeInTheDocument();
    });

    it("requires a brand", async () => {
        const onSubmit = vi.fn();

        renderWithContext(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("brandIsRequired")).toBeInTheDocument();
    });

    it("requires a name", async () => {
        const onSubmit = vi.fn();

        renderWithContext(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("nameIsRequired")).toBeInTheDocument();
    });
});

describe("Submitting the form with no Product provided", () => {
    it.skip("should be tested");
});

describe("Submitting the form with a Product provided", () => {
    it("should call the onSubmit handler with the form data and the ID of the provided Product", async () => {
        const onSubmit = vi.fn();
        const expectedProduct = fakeProduct();

        renderWithContext(<ProductForm onSubmit={onSubmit} product={expectedProduct} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).toHaveBeenCalledWith(expectedProduct);
    });
});
