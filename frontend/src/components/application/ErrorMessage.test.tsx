import { it, expect } from "vitest";
import { ErrorMessage } from "./ErrorMessage";
import { render, screen } from "@testing-library/react";

it("should not render anything if there are no children", () => {
    const { container } = render(<ErrorMessage />);

    expect(container.firstChild).toBeNull();
});

it("should render the children", () => {
    render(<ErrorMessage>error message</ErrorMessage>);

    expect(screen.getByText("error message")).toBeInTheDocument();
});
