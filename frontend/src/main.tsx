import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.scss";
import BackendProductService from "./services/BackendProductService";

const productService = new BackendProductService({ baseUrl: "/api" });
(window as unknown as Record<string, unknown>).productService = productService;

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
