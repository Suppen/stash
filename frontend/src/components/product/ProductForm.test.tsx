import { describe, it, expect, vi } from "vitest";
import { ProductForm } from "./ProductForm";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { fakeProduct } from "../../domain/entities/fakeProduct";

describe("Default values", () => {
    it("should not have a default value for ID", () => {
        render(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("id")).toHaveValue("");
    });

    it("should not have a default value for brand", () => {
        render(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("brand")).toHaveValue("");
    });

    it("should not have a default value for name", () => {
        render(<ProductForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("name")).toHaveValue("");
    });
});

describe("Providing a product", () => {
    it("should have the provided ID", () => {
        const product = fakeProduct();
        render(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("id")).toHaveValue(product.id.value());
    });

    it("should have the provided brand", () => {
        const product = fakeProduct();
        render(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("brand")).toHaveValue(product.brand.value());
    });

    it("should have the provided name", () => {
        const product = fakeProduct();
        render(<ProductForm onSubmit={vi.fn()} product={product} />);

        expect(screen.getByLabelText("name")).toHaveValue(product.name);
    });
});

describe("Validation", () => {
    it("requires an ID", async () => {
        const onSubmit = vi.fn();

        render(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("idIsRequired")).toBeInTheDocument();
    });

    it("requires a brand", async () => {
        const onSubmit = vi.fn();

        render(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("brandIsRequired")).toBeInTheDocument();
    });

    it("requires a name", async () => {
        const onSubmit = vi.fn();

        render(<ProductForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("nameIsRequired")).toBeInTheDocument();
    });
});

describe("Submitting the form with no Product provided", () => {
    it("should call the onSubmit handler with the form data", async () => {
        const onSubmit = vi.fn();
        const expectedProduct = fakeProduct();

        render(<ProductForm onSubmit={onSubmit} />);

        await userEvent.type(screen.getByLabelText("id"), expectedProduct.id.value());
        await userEvent.type(screen.getByLabelText("brand"), expectedProduct.brand.value());
        await userEvent.type(screen.getByLabelText("name"), expectedProduct.name);
        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).toHaveBeenCalledWith({
            id: expectedProduct.id,
            brand: expectedProduct.brand,
            name: expectedProduct.name,
            stashItems: []
        });
    });
});

describe("Submitting the form with a Product provided", () => {
    it("should call the onSubmit handler with the form data and the ID of the provided Product", async () => {
        const onSubmit = vi.fn();
        const expectedProduct = fakeProduct();

        render(<ProductForm onSubmit={onSubmit} product={expectedProduct} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).toHaveBeenCalledWith(expectedProduct);
    });
});
