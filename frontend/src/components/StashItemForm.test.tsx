import { describe, it, expect, vi, beforeEach } from "vitest";
import { StashItemForm } from "./StashItemForm";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { fakeStashItem } from "../domain/entities/fakeStashItem";
import { StashItem } from "../domain/entities/StashItem";
import { UUID } from "../domain/valueObjects/UUID";

let user: ReturnType<typeof userEvent.setup>;
beforeEach(() => {
    user = userEvent.setup();
});

describe("Default values", () => {
    it("should have default value for quantity", () => {
        render(<StashItemForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("quantity")).toHaveValue(1);
    });

    it("should not have a default value for expiry date", () => {
        render(<StashItemForm onSubmit={vi.fn()} />);

        expect(screen.getByLabelText("expiryDate")).toHaveValue("");
    });
});

describe("Providing a stash item", () => {
    it("should have the provided quantity", () => {
        const stashItem = fakeStashItem();
        render(<StashItemForm onSubmit={vi.fn()} stashItem={stashItem} />);

        expect(screen.getByLabelText("quantity")).toHaveValue(stashItem.quantity.value());
    });

    it("should have the provided expiry date", () => {
        const stashItem = fakeStashItem();
        render(<StashItemForm onSubmit={vi.fn()} stashItem={stashItem} />);

        expect(screen.getByLabelText("expiryDate")).toHaveValue(stashItem.expiryDate.toISOString());
    });
});

describe("Validation", () => {
    it("should not allow a quantity of zero", async () => {
        const onSubmit = vi.fn();

        render(<StashItemForm onSubmit={onSubmit} />);

        const quantityField = screen.getByLabelText("quantity");
        await user.clear(quantityField);
        await user.type(quantityField, "0");
        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("quantityMustBeGreaterThanZero")).toBeInTheDocument();
    });

    it("requires a quantity", async () => {
        const onSubmit = vi.fn();

        render(<StashItemForm onSubmit={onSubmit} />);

        const quantityField = screen.getByLabelText("quantity");
        await user.clear(quantityField);
        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("quantityIsRequired")).toBeInTheDocument();
    });

    it("requires an expiry date", async () => {
        const onSubmit = vi.fn();

        render(<StashItemForm onSubmit={onSubmit} />);

        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).not.toHaveBeenCalled();
        expect(screen.getByText("expiryDateIsRequired")).toBeInTheDocument();
    });
});

describe("Submitting the form with no StashItem provided", () => {
    it("should call the onSubmit handler with a StashItem made from the form data", async () => {
        const onSubmit = vi.fn();
        const expectedStashItem = fakeStashItem();

        render(<StashItemForm onSubmit={onSubmit} stashItem={expectedStashItem} />);

        // Manipulate the quantity field
        const quantityInput = screen.getByLabelText("quantity");
        await user.clear(quantityInput);
        await user.type(quantityInput, expectedStashItem.quantity.value().toString());

        // Manipulate the expiry date field
        const expiryDateInput: HTMLInputElement = screen.getByLabelText("expiryDate");
        await user.clear(expiryDateInput);
        await user.type(expiryDateInput, expectedStashItem.expiryDate.toISOString());

        // Submit the form
        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).toHaveBeenCalled();
        const submittedStashItem = (onSubmit.mock.calls[0] as unknown[])[0] as StashItem;
        expect(submittedStashItem.id).toBeInstanceOf(UUID);
        expect(submittedStashItem.quantity).toEqual(expectedStashItem.quantity);
        expect(submittedStashItem.expiryDate).toEqual(expectedStashItem.expiryDate);
    });
});

describe("Submitting the form with a StashItem provided", () => {
    it("should call the onSubmit handler with the provided StashItem", async () => {
        const onSubmit = vi.fn();
        const expectedStashItem = fakeStashItem();

        render(<StashItemForm onSubmit={onSubmit} stashItem={expectedStashItem} />);

        // Submit the form
        await userEvent.click(screen.getByText("save"));

        expect(onSubmit).toHaveBeenCalledWith(expectedStashItem);
    });
});
