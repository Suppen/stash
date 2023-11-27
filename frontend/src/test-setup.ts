import { cleanup } from "@testing-library/react";
import { expect, afterEach } from "vitest";
import matchers from "@testing-library/jest-dom/matchers";

// Add DOM matchers to `expect`
expect.extend(matchers);

// Clear the DOM after each test
afterEach(() => {
    cleanup();
});
