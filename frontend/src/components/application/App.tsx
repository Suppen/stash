import { useMemo } from "react";
import BackendProductService from "../../services/BackendProductService";
import { ProductService } from "../../services/ProductService";
import "../../i18n/i18n";
import { ProductForm } from "../product/ProductForm";

const App = (): JSX.Element => {
    const productService: ProductService = useMemo(() => new BackendProductService({ baseUrl: "/api" }), []);

    return (
        <div className="app">
            <ProductForm onSubmit={console.log} />
        </div>
    );
};

export default App;
