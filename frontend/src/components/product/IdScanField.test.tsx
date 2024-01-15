import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { IdScanField } from "./IdScanField";
import { fakeProductId } from "../../domain/valueObjects/fakeProductId";
import userEvent from "@testing-library/user-event";

describe("Entering an ID", () => {
    it("should call the onScan callback with the entered ID", async () => {
        const productId = fakeProductId();
        const onScan = vi.fn();

        render(<IdScanField onScan={onScan} />);

        const input = screen.getByRole("textbox");
        const user = userEvent.setup();

        await user.type(input, productId.value());
        await user.type(input, "{enter}");

        expect(onScan).toHaveBeenCalledWith(productId);
    });
});
