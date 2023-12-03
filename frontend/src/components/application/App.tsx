import { useMemo } from "react";
import BackendProductService from "../../services/BackendProductService";
import { ProductService } from "../../services/ProductService";
import { StashItemForm } from "../../components/StashItemForm";
import "../../i18n/i18n";

const App = (): JSX.Element => {
    const productService: ProductService = useMemo(() => new BackendProductService({ baseUrl: "/api" }), []);

    return (
        <div className="app">
            <StashItemForm onSubmit={console.log} />
        </div>
    );
};

export default App;
