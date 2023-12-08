import { useMemo } from "react";
import BackendProductService from "../../services/BackendProductService";
import { ProductService } from "../../services/ProductService";
import "../../i18n/i18n";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Router } from "./Router";

const App = (): JSX.Element => {
    const queryClient = new QueryClient();

    const productService: ProductService = useMemo(() => new BackendProductService({ baseUrl: "/api" }), []);

    return (
        <QueryClientProvider client={queryClient}>
            <div className="app">
                <Router productService={productService} />
            </div>
        </QueryClientProvider>
    );
};

export default App;
