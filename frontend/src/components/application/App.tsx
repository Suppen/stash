import { useMemo } from "react";
import BackendProductService from "../../services/BackendProductService";
import { ProductService } from "../../services/ProductService";
import "../../i18n/i18n";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Router } from "./Router";
import { AutoFullscreen } from "./AutoFullscreen";

const App = (): JSX.Element => {
    const queryClient = new QueryClient();

    const productService: ProductService = useMemo(() => new BackendProductService({ baseUrl: "/api" }), []);

    (window as unknown as Record<string, unknown>).productService = productService;

    return (
        <QueryClientProvider client={queryClient}>
            {window.screen.width === 800 ? <AutoFullscreen /> : null}
            <div className="app">
                <Router productService={productService} />
            </div>
        </QueryClientProvider>
    );
};

export default App;
